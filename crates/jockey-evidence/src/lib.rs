//! JOCKY Evidence — Evidence Model, Receipts, Provenance, Verification, Export
//! 
//! Phase 2: Stub implementation for CI validation.
//! Actual implementation begins in Phase 3.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Re-exports for public API
pub mod model;
pub mod receipt;
pub mod provenance;
pub mod verification;
pub mod export;

/// Evidence Model module
pub mod model {
    /// Evidence placeholder
    pub struct Evidence;
    
    /// Evidence Receipt placeholder
    pub struct EvidenceReceipt;
    
    /// Provenance Record placeholder
    pub struct ProvenanceRecord;
    
    /// Finding placeholder
    pub struct Finding;
    
    /// Evidence Type enum placeholder
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
}

/// Receipt module
pub mod receipt {
    /// Sign receipt
    pub fn sign_receipt(_evidence: &crate::model::Evidence, _key: &[u8]) -> crate::model::EvidenceReceipt {
        unimplemented!("Phase 3 implementation")
    }
    
    /// Verify receipt
    pub fn verify_receipt(_receipt: &crate::model::EvidenceReceipt, _evidence: &crate::model::Evidence) -> Result<(), VerifyError> {
        unimplemented!("Phase 3 implementation")
    }
    
    /// Verify error type
    #[derive(Debug)]
    pub struct VerifyError;
}

/// Provenance module
pub mod provenance {
    /// Provenance record placeholder
    pub struct ProvenanceRecord;
    
    /// Verify provenance chain
    pub fn verify_provenance_chain(_chain: &[crate::model::ProvenanceRecord]) -> Result<(), VerifyError> {
        unimplemented!("Phase 3 implementation")
    }
    
    /// Verify error type
    #[derive(Debug)]
    pub struct VerifyError;
}

/// Verification module
pub mod verification {
    /// Verification engine placeholder
    pub struct VerificationEngine;
    
    /// Verification report placeholder
    pub struct VerificationReport;
}

/// Export module
pub mod export {
    /// Export format enum
    pub enum ExportFormat {
        CaseUco,
        Cytoscape,
        Timesketch,
        Markdown,
    }
    
    /// Exporter placeholder
    pub struct Exporter;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stub_compiles() {
        assert!(true);
    }
}