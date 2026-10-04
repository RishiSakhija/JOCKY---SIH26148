//! JOCKY Collectors — Collector Trait, Windows/Linux Implementations
//! 
//! Phase 2: Stub implementation for CI validation.
//! Actual implementation begins in Phase 4.

#![warn(missing_docs)]
#![warn(clippy::all)]

use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

/// Re-exports for public API
pub mod trait_def;
pub mod registry;
pub mod windows;
pub mod linux;

/// Collector Trait definition
pub mod trait_def {
    use super::*;
    
    /// Collector trait
    #[async_trait]
    pub trait Collector: Send + Sync {
        fn name(&self) -> &'static str;
        fn version(&self) -> &'static str;
        fn supported_actions(&self) -> Vec<&'static str>;
        fn supported_os(&self) -> Vec<&'static str>;
        async fn execute(
            &self,
            action: &str,
            params: Value,
            ctx: &ExecutionContext,
        ) -> Result<CollectorOutput, CollectorError>;
    }
    
    /// Execution Context
    pub struct ExecutionContext {
        pub run_id: String,
        pub evidence_root: PathBuf,
        pub target: TargetSpec,
        pub temp_dir: PathBuf,
        pub credentials: Option<Value>,
    }
    
    /// Target Specification
    pub struct TargetSpec {
        pub target_type: String,
        pub identifier: String,
        pub os: String,
        pub arch: String,
    }
    
    /// Collector Output
    pub struct CollectorOutput {
        pub evidence: Vec<crate::evidence::Evidence>,
        pub artifacts: Vec<ArtifactRef>,
        pub metrics: CollectorMetrics,
    }
    
    /// Artifact Reference
    pub struct ArtifactRef {
        pub path: String,
        pub type_: String,
        pub description: String,
    }
    
    /// Collector Metrics
    pub struct CollectorMetrics {
        pub duration_ms: u64,
        pub bytes_read: u64,
        pub bytes_written: u64,
        pub errors: Vec<String>,
    }
    
    /// Collector Error
    #[derive(Debug, thiserror::Error)]
    pub enum CollectorError {
        #[error("collector not found: {0}")]
        NotFound(String),
        #[error("permission denied: {0}")]
        PermissionDenied(String),
        #[error("invalid parameter: {0}")]
        InvalidParameter(String),
        #[error("timeout")]
        Timeout,
        #[error("internal error: {0}")]
        Internal(String),
        #[error("unsupported action: {0}")]
        UnsupportedAction(String),
        #[error("unsupported OS: {0}")]
        UnsupportedOS(String),
    }
}

/// Collector Registry module
pub mod registry {
    use std::collections::HashMap;
    use super::trait_def::*;
    
    /// Collector Manifest
    pub struct CollectorManifest {
        pub name: String,
        pub version: String,
        pub supported_os: Vec<String>,
        pub capabilities: Vec<String>,
        pub actions: HashMap<String, ActionManifest>,
        pub public_key: String,
    }
    
    /// Action Manifest
    pub struct ActionManifest {
        pub capabilities: Vec<String>,
        pub params_schema: serde_json::Value,
        pub description: String,
    }
    
    /// Collector Registry
    pub struct CollectorRegistry {
        collectors: HashMap<String, Box<dyn Collector>>,
        manifests: HashMap<String, CollectorManifest>,
    }
    
    impl CollectorRegistry {
        pub fn new() -> Self {
            Self {
                collectors: HashMap::new(),
                manifests: HashMap::new(),
            }
        }
        
        pub fn register(&mut self, collector: Box<dyn Collector>) -> Result<(), RegistryError> {
            unimplemented!("Phase 4 implementation")
        }
        
        pub fn get(&self, name: &str) -> Option<&dyn Collector> {
            unimplemented!("Phase 4 implementation")
        }
        
        pub fn get_for_os(&self, os: &str) -> Vec<&dyn Collector> {
            unimplemented!("Phase 4 implementation")
        }
        
        pub fn get_manifest(&self, name: &str) -> Option<&CollectorManifest> {
            unimplemented!("Phase 4 implementation")
        }
    }
    
    /// Registry Error
    #[derive(Debug, thiserror::Error)]
    pub enum RegistryError {
        #[error("collector already registered: {0}")]
        AlreadyRegistered(String),
        #[error("collector not found: {0}")]
        NotFound(String),
        #[error("invalid manifest: {0}")]
        InvalidManifest(String),
    }
}

/// Windows collectors module
pub mod windows {
    /// Windows collector stubs
    pub mod proc_windows;
    pub mod file_windows;
    pub mod registry_windows;
    pub mod logs_windows;
    pub mod net_windows;
}

/// Linux collectors module
pub mod linux {
    /// Linux collector stubs
    pub mod proc_linux;
    pub mod file_linux;
    pub mod config_linux;
    pub mod logs_linux;
    pub mod net_linux;
}

/// Re-export Evidence type from jockey-evidence
pub use crate::evidence::Evidence;

/// Evidence placeholder for cross-crate reference
pub mod evidence {
    pub struct Evidence;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stub_compiles() {
        assert!(true);
    }
}