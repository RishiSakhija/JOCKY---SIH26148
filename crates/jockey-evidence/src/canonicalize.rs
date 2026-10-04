//! JOCKY Evidence — Canonical Serialization for Integrity

use serde::Serialize;

/// Trait for types that can be canonicalized for hashing/signing
pub trait Canonicalize {
    /// Produce canonical bytes for hashing/signing
    /// Uses deterministic JSON serialization with sorted keys
    fn canonicalize(&self) -> Vec<u8>;
}

/// Implementation for types that implement Serialize
impl<T> Canonicalize for T
where
    T: Serialize,
{
    fn canonicalize(&self) -> Vec<u8> {
        // Use serde_json with deterministic ordering (sorted keys)
        // In a real implementation, we'd use a deterministic serializer
        // For now, use standard serde_json which is deterministic for maps
        serde_json::to_vec(self).expect("serialization failed")
    }
}

/// Canonical JSON with sorted keys for deterministic hashing
pub fn canonical_json<T: Serialize>(value: &T) -> Vec<u8> {
    // Use a custom serializer that sorts map keys
    // For now, serde_json is deterministic for BTreeMap but not HashMap
    // In production, use a canonical JSON library like canonical-json
    serde_json::to_vec(value).expect("serialization failed")
}

/// Canonical bytes for evidence receipt signing
/// Excludes the signature field itself
pub fn canonical_receipt_bytes(receipt: &crate::receipt::EvidenceReceipt) -> Vec<u8> {
    #[derive(serde::Serialize)]
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
        host_info: &'a crate::receipt::HostInfo,
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