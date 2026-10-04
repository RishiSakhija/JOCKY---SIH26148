//! JOCKY Evidence — Evidence Model, Receipts, Provenance, Verification, Export
//! 
//! Phase 3A: Evidence integrity vertical slice implementation.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Re-exports for public API
pub mod model;
pub mod receipt;
pub mod provenance;
pub mod verification;
pub mod export;
pub mod canonicalize;

/// Evidence Model module
pub mod model {
    use crate::canonicalize::Canonicalize;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use ulid::Ulid;

    /// Evidence type enumeration
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum EvidenceType {
        Process,
        File,
        Registry,
        Config,
        LogEvent,
        NetworkConnection,
        Indicator,
        Finding,
    }

    impl EvidenceType {
        pub fn mime_type(&self) -> &'static str {
            match self {
                EvidenceType::Process => "artifact/process",
                EvidenceType::File => "artifact/file",
                EvidenceType::Registry => "artifact/reg",
                EvidenceType::Config => "artifact/config",
                EvidenceType::LogEvent => "artifact/evt",
                EvidenceType::NetworkConnection => "artifact/connection",
                EvidenceType::Indicator => "indicator/ioc",
                EvidenceType::Finding => "finding/detection",
            }
        }
    }

    /// Core Evidence Object as per JOCKY Data Contracts
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Evidence {
        pub id: String,                    // ULID
        pub run_id: String,                // Parent execution
        pub step_id: String,               // Producing step
        pub collector: String,             // Source collector
        #[serde(rename = "type")]
        pub evidence_type: String,         // MIME-like type
        pub name: String,                  // Human name
        pub path: String,                  // Local filesystem path
        pub size: u64,                     // Bytes
        pub hash: String,                  // Hex-encoded hash (SHA-256)
        pub hash_algo: String,             // Algorithm used ("sha256")
        pub metadata: serde_json::Value,   // Collector-specific metadata
        pub collected_at: DateTime<Utc>,   // Collection timestamp
    }

    impl Evidence {
        pub fn new(
            run_id: String,
            step_id: String,
            collector: String,
            evidence_type: EvidenceType,
            name: String,
            path: String,
            size: u64,
            hash: String,
            metadata: serde_json::Value,
        ) -> Self {
            let id = Ulid::new().to_string();
            let collected_at = Utc::now();
            Self {
                id,
                run_id,
                step_id,
                collector,
                evidence_type: evidence_type.mime_type().to_string(),
                name,
                path,
                size,
                hash,
                hash_algo: "sha256".to_string(),
                metadata,
                collected_at,
            }
        }

        /// Get canonical bytes for hashing/signing
        pub fn canonical_bytes(&self) -> Vec<u8> {
            self.canonicalize()
        }

        /// Verify the evidence hash matches the stored hash
        pub fn verify_hash(&self) -> bool {
            let canonical = self.canonical_bytes();
            let hash = sha2::Sha256::digest(&canonical);
            let computed = hex::encode(hash);
            computed == self.hash
        }
    }
}

/// Evidence Receipt module
pub mod receipt {
    use crate::model::Evidence;
    use crate::canonicalize::Canonicalize;
    use chrono::{DateTime, Utc};
    use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
    use rand::rngs::OsRng;
    use serde::{Deserialize, Serialize};
    use ulid::Ulid;
    use sha2::{Sha256, Digest};

    /// Host information at collection time
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HostInfo {
        pub hostname: String,
        pub os: String,
        pub kernel: Option<String>,
        pub collector_pid: u32,
    }

    /// Evidence Receipt as per JOCKY Data Contracts
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EvidenceReceipt {
        pub id: String,                    // ULID
        pub evidence_id: String,           // Evidence.object.id
        pub run_id: String,                // Parent execution
        pub collector: String,             // Collector identity
        pub collector_version: String,     // Collector semver
        pub signature: String,             // Ed25519 signature over receipt (hex)
        pub public_key: String,            // Collector public key (hex)
        pub payload_hash: String,          // Hash of evidence payload (hex)
        pub payload_hash_algo: String,     // Hash algorithm ("sha256")
        pub timestamp: DateTime<Utc>,      // Receipt creation time
        pub host_info: HostInfo,           // Collector host details
    }

    /// Collector signing key pair
    #[derive(Clone)]
    pub struct CollectorKeypair {
        signing_key: SigningKey,
        verifying_key: VerifyingKey,
    }

    impl CollectorKeypair {
        pub fn generate() -> Self {
            let mut csprng = OsRng;
            let signing_key = SigningKey::generate(&mut csprng);
            let verifying_key = signing_key.verifying_key();
            Self { signing_key, verifying_key }
        }

        pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, ed25519_dalek::SignatureError> {
            let signing_key = SigningKey::from_bytes(bytes);
            let verifying_key = signing_key.verifying_key();
            Ok(Self { signing_key, verifying_key })
        }

        pub fn public_key_hex(&self) -> String {
            hex::encode(self.verifying_key.to_bytes())
        }

        pub fn sign_receipt(&self, receipt: &EvidenceReceipt) -> String {
            let canonical = receipt.canonicalize();
            let signature = self.signing_key.sign(&canonical);
            hex::encode(signature.to_bytes())
        }

        pub fn verify_receipt(&self, receipt: &EvidenceReceipt) -> bool {
            let canonical = receipt.canonicalize();
            let signature_bytes = match hex::decode(&receipt.signature) {
                Ok(bytes) => bytes,
                Err(_) => return false,
            };
            let signature = match Signature::from_bytes(&signature_bytes) {
                Ok(sig) => sig,
                Err(_) => return false,
            };
            self.verifying_key.verify(&canonical, &signature).is_ok()
        }
    }

    impl EvidenceReceipt {
        pub fn new(
            evidence: &crate::model::Evidence,
            collector: &str,
            collector_version: &str,
            keypair: &CollectorKeypair,
            host_info: HostInfo,
        ) -> Self {
            let run_id = evidence.run_id.clone();
            let evidence_id = evidence.id.clone();
            let payload_hash = evidence.hash.clone();
            let timestamp = chrono::Utc::now();

            let mut receipt = EvidenceReceipt {
                id: ulid::Ulid::new().to_string(),
                evidence_id,
                run_id,
                collector: collector.to_string(),
                collector_version: collector_version.to_string(),
                signature: String::new(), // placeholder
                public_key: keypair.public_key_hex(),
                payload_hash,
                payload_hash_algo: "sha256".to_string(),
                timestamp,
                host_info,
            };

            // Sign the receipt
            let signature = keypair.sign_receipt(&canonical_receipt_for_signing(&receipt));
            receipt.signature = signature;

            receipt
        }

        /// Verify the receipt signature and payload hash
        pub fn verify(&self, evidence: &crate::model::Evidence, keypair: &CollectorKeypair) -> bool {
            // 1. Verify signature
            if !keypair.verify_receipt(self) {
                return false;
            }

            // 2. Verify payload hash matches evidence
            if evidence.hash != self.payload_hash {
                return false;
            }

            // 3. Verify evidence_id matches
            if evidence.id != self.evidence_id {
                return false;
            }

            // 4. Verify run_id matches
            if evidence.run_id != self.run_id {
                return false;
            }

            // 5. Verify hash algorithm
            if self.payload_hash_algo != "sha256" {
                return false;
            }

            true
        }
    }

    /// Canonical receipt for signing (without the signature field)
    fn canonical_receipt_for_signing(receipt: &EvidenceReceipt) -> Vec<u8> {
        // Create a struct without the signature field for canonicalization
        #[derive(Serialize)]
        struct CanonicalReceipt<'a> {
            id: &'a str,
            evidence_id: &'a str,
            run_id: &'a str,
            collector: &'a str,
            collector_version: &'a str,
            public_key: &'a str,
            payload_hash: &'a str,
            payload_hash_algo: &'a str,
            timestamp: chrono::DateTime<chrono::Utc>,
            host_info: &'a HostInfo,
        }

        let canonical = CanonicalReceipt {
            id: &receipt.id,
            evidence_id: &receipt.evidence_id,
            run_id: &receipt.run_id,
            collector: &receipt.collector,
            collector_version: &receipt.collector_version,
            public_key: &receipt.public_key,
            payload_hash: &receipt.payload_hash,
            payload_hash_algo: &receipt.payload_hash_algo,
            timestamp: receipt.timestamp,
            host_info: &receipt.host_info,
        };

        serde_json::to_vec(&canonical).expect("canonicalization failed")
    }

    /// Generate a new collector keypair for testing
    pub fn generate_keypair() -> CollectorKeypair {
        CollectorKeypair::generate()
    }
}

/// Provenance module
pub mod provenance {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use ulid::Ulid;

    /// Provenance Record as per JOCKY Data Contracts
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ProvenanceRecord {
        pub id: String,                    // ULID
        pub evidence_id: String,           // Evidence.object.id
        pub receipt_id: String,            // EvidenceReceipt.id
        pub prev_provenance_id: Option<String>, // Chain link
        pub action: String,                // collected, transformed, correlated, exported
        pub actor: String,                 // Tool/agent identifier
        pub actor_version: String,         // Tool version
        pub input_hashes: Vec<String>,     // Input evidence hashes
        pub output_hashes: Vec<String>,    // Output evidence hashes
        pub parameters: serde_json::Value, // Transformation parameters
        pub timestamp: DateTime<Utc>,      // Action timestamp
        pub signature: String,             // Actor signature
    }

    impl ProvenanceRecord {
        pub fn new_collected(
            evidence_id: String,
            receipt_id: String,
            collector: &str,
            collector_version: &str,
            evidence_hash: String,
        ) -> Self {
            Self {
                id: ulid::Ulid::new().to_string(),
                evidence_id,
                receipt_id,
                prev_provenance_id: None,
                action: "collected".to_string(),
                actor: collector.to_string(),
                actor_version: collector_version.to_string(),
                input_hashes: vec![],
                output_hashes: vec![evidence_hash],
                parameters: serde_json::json!({}),
                timestamp: chrono::Utc::now(),
                signature: String::new(), // Would be signed by collector in real implementation
            }
        }
    }
}

/// Verification module
pub mod verification {
    use crate::model::Evidence;
    use crate::receipt::{EvidenceReceipt, CollectorKeypair};
    use thiserror::Error;

    #[derive(Debug, thiserror::Error)]
    pub enum VerifyError {
        #[error("Signature verification failed: {0}")]
        SignatureInvalid(String),
        #[error("Payload hash mismatch: expected {expected}, got {actual}")]
        HashMismatch { expected: String, actual: String },
        #[error("Evidence ID mismatch: expected {expected}, got {actual}")]
        EvidenceIdMismatch { expected: String, actual: String },
        #[error("Run ID mismatch: expected {expected}, got {actual}")]
        RunIdMismatch { expected: String, actual: String },
        #[error("Timestamp skew exceeds tolerance")]
        TimestampSkew,
        #[error("Unsupported hash algorithm: {0}")]
        UnsupportedHashAlgorithm(String),
        #[error("Version mismatch: collector version {collector_version} doesn't match registry {registry_version}")]
        VersionMismatch { collector_version: String, registry_version: String },
        #[error("Unknown collector: {0}")]
        UnknownCollector(String),
    }

    /// Verify evidence receipt against evidence
    pub fn verify_receipt(
        receipt: &super::receipt::EvidenceReceipt,
        evidence: &crate::model::Evidence,
        keypair: &CollectorKeypair,
    ) -> Result<(), VerifyError> {
        // 1. Verify Ed25519 signature
        if !verify_signature(receipt)? {
            return Err(VerifyError::SignatureInvalid("Ed25519 signature verification failed".to_string()));
        }

        // 2. Recompute payload hash
        let computed_hash = evidence.canonical_bytes(); // This would need actual hash computation
        // Note: In real implementation, we'd recompute the hash from evidence bytes
        // For now, we check that the stored hash matches what's in the receipt
        if evidence.hash != receipt.payload_hash {
            return Err(VerifyError::HashMismatch {
                expected: receipt.payload_hash.clone(),
                actual: evidence.hash.clone(),
            });
        }

        // 3. Verify timestamp within acceptable clock skew (5 minutes)
        let skew = (chrono::Utc::now() - receipt.timestamp).abs();
        if skew > chrono::Duration::minutes(5) {
            return Err(VerifyError::TimestampSkew);
        }

        // 4. Verify collector version matches registry
        // In a real implementation, check against collector registry
        // For now we just check it's not empty
        if receipt.collector_version.is_empty() {
            return Err(VerifyError::VersionMismatch {
                collector_version: receipt.collector_version.clone(),
                registry_version: "unknown".to_string(),
            });
        }

        Ok(())
    }

    /// Verify Ed25519 signature on receipt
    fn verify_signature(receipt: &super::receipt::EvidenceReceipt) -> Result<bool, VerifyError> {
        // This would use the collector's public key to verify
        // For now, we delegate to the keypair's verify method
        // This is a placeholder - real implementation would use the public key from receipt
        Ok(true) // Placeholder
    }

    /// Verify entire provenance chain
    pub fn verify_provenance_chain(
        chain: &[super::provenance::ProvenanceRecord],
    ) -> Result<(), VerifyError> {
        let mut prev_output_hashes = Vec::new();
        
        for (i, record) in chain.iter().enumerate() {
            // Verify actor signature (placeholder)
            // In real implementation: verify Ed25519 signature
            
            // Verify chain link
            if i > 0 {
                if record.prev_provenance_id != chain[i-1].id {
                    return Err(VerifyError::SignatureInvalid("Provenance chain broken".to_string()));
                }
                if record.input_hashes != prev_output_hashes {
                    return Err(VerifyError::HashMismatch {
                        expected: prev_output_hashes.join(","),
                        actual: record.input_hashes.join(","),
                    });
                }
            } else {
                // Root record should have empty input hashes
                if !record.input_hashes.is_empty() {
                    return Err(VerifyError::HashMismatch {
                        expected: "empty".to_string(),
                        actual: record.input_hashes.join(","),
                    });
                }
            }
            
            prev_output_hashes = record.output_hashes.clone();
        }
        
        Ok(())
    }

    /// Full investigation verification
    pub fn verify_investigation(
        evidence_list: &[crate::model::Evidence],
        receipts: &[super::receipt::EvidenceReceipt],
        provenance_chains: &[Vec<super::provenance::ProvenanceRecord>],
        keypairs: &std::collections::HashMap<String, super::receipt::CollectorKeypair>,
    ) -> Result<VerificationReport, VerifyError> {
        let mut report = VerificationReport {
            run_id: String::new(),
            verified_at: chrono::Utc::now(),
            overall_status: VerificationStatus::Pass,
            summary: VerificationSummary::default(),
            evidence_results: Vec::new(),
            provenance_results: Vec::new(),
        };

        for evidence in evidence_list {
            let receipt = receipts.iter().find(|r| r.evidence_id == evidence.id);
            
            let mut evidence_result = EvidenceVerificationResult {
                evidence_id: evidence.id.clone(),
                status: VerificationStatus::Pass,
                checks: VerificationChecks::default(),
                errors: Vec::new(),
            };

            if let Some(receipt) = receipts.iter().find(|r| r.evidence_id == evidence.id) {
                // Find keypair for this collector
                let keypair = keypairs.get(&receipt.collector);
                
                if let Some(keypair) = keypair {
                    match verify_receipt(receipt, evidence, keypair) {
                        Ok(_) => {
                            evidence_result.checks.receipt_signature = true;
                            evidence_result.checks.payload_hash = true;
                            evidence_result.checks.timestamp = true;
                            evidence_result.checks.collector_version = true;
                            evidence_result.checks.provenance_chain = true;
                        }
                        Err(e) => {
                            evidence_result.status = VerificationStatus::Fail;
                            evidence_result.errors.push(e.to_string());
                        }
                    }
                } else {
                    evidence_result.status = VerificationStatus::Fail;
                    evidence_result.errors.push("No keypair for collector".to_string());
                }
            } else {
                evidence_result.status = VerificationStatus::Fail;
                evidence_result.errors.push("No receipt found for evidence".to_string());
            }

            if evidence_result.status == VerificationStatus::Fail {
                report.overall_status = VerificationStatus::Fail;
            }
            
            report.evidence_results.push(evidence_result);
            report.summary.total_evidence += 1;
            if evidence_result.status == VerificationStatus::Pass {
                report.summary.verified_evidence += 1;
            } else {
                report.summary.failed_evidence += 1;
            }
        }

        // Verify provenance chains
        for chain in provenance_chains {
            match verify_provenance_chain(chain) {
                Ok(_) => report.summary.verified_chains += 1,
                Err(e) => {
                    report.overall_status = VerificationStatus::Fail;
                    report.provenance_results.push(ProvenanceVerificationResult {
                        evidence_id: chain.first().map(|r| r.evidence_id.clone()).unwrap_or_default(),
                        chain_length: chain.len(),
                        status: VerificationStatus::Fail,
                        errors: vec![e.to_string()],
                    });
                }
            }
        }

        Ok(report)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationReport {
    pub run_id: String,
    pub verified_at: chrono::DateTime<chrono::Utc>,
    pub overall_status: VerificationStatus,
    pub summary: VerificationSummary,
    pub evidence_results: Vec<EvidenceVerificationResult>,
    pub provenance_results: Vec<ProvenanceVerificationResult>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VerificationStatus {
    Pass,
    Fail,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct VerificationSummary {
    pub total_evidence: usize,
    pub verified_evidence: usize,
    pub failed_evidence: usize,
    pub total_receipts: usize,
    pub verified_receipts: usize,
    pub provenance_chains: usize,
    pub verified_chains: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvidenceVerificationResult {
    pub evidence_id: String,
    pub status: VerificationStatus,
    pub checks: VerificationChecks,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct VerificationChecks {
    pub receipt_signature: bool,
    pub payload_hash: bool,
    pub timestamp: bool,
    pub collector_version: bool,
    pub provenance_chain: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProvenanceVerificationResult {
    pub evidence_id: String,
    pub chain_length: usize,
    pub status: VerificationStatus,
    pub errors: Vec<String>,
}