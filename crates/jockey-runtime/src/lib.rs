//! JOCKY Runtime — Execution Engine, Dispatcher, Sandbox

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Re-exports for public API
pub mod executor;
pub mod dispatcher;
pub mod sandbox;
pub mod collector_registry;

/// Execute a contract
pub async fn execute(
    contract: crate::contract::ExecutionContract,
    registry: &crate::collector_registry::CollectorRegistry,
) -> Result<crate::executor::ExecutionResult, RuntimeError> {
    let executor = crate::executor::Executor::new(registry);
    executor.execute(contract).await
}

/// Runtime error
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Contract validation failed: {0}")]
    ContractValidation(String),
    #[error("Collector error: {0}")]
    CollectorError(#[from] crate::collectors::trait_def::CollectorError),
    #[error("Evidence storage error: {0}")]
    StorageError(String),
    #[error("Collector not found: {0}")]
    CollectorNotFound(String),
    #[error("Policy violation: {0}")]
    PolicyViolation(String),
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub run_id: String,
    pub status: ExecutionStatus,
    pub evidence_collected: usize,
    pub receipts_generated: usize,
    pub provenance_records: usize,
    pub findings: usize,
    pub duration_ms: u64,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Success,
    Partial,
    Failed,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stub_compiles() {
        assert!(true);
    }
}