//! JOCKY Core — IR, Execution Contract, Policy Engine, Storage
//! 
//! Core types shared across the JOCKY platform.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Re-exports for public API
pub mod ir;
pub mod contract;
pub mod policy;
pub mod storage;
pub mod target;
pub mod capability;
pub mod collector;

/// IR (Intermediate Representation) module
pub mod ir {
    use serde::{Deserialize, Serialize};
    use chrono::{DateTime, Utc};
    
    /// IR Module
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IrModule {
        pub version: u32,
        pub id: String,
        pub metadata: IRMetadata,
        pub steps: Vec<IRStep>,
        pub outputs: Vec<IROutput>,
    }
    
    /// IR Metadata
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IRMetadata {
        pub name: String,
        pub description: Option<String>,
        pub author: Option<String>,
        pub created_at: DateTime<Utc>,
        pub tags: Vec<String>,
    }
    
    /// IR Step
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IRStep {
        pub id: String,
        pub collector: String,
        pub action: String,
        pub params: serde_json::Value,
        pub binds: Vec<String>,
        pub depends_on: Vec<String>,
        pub condition: Option<String>,
    }
    
    /// IR Output
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IROutput {
        pub name: String,
        pub from_step: String,
        pub field: Option<String>,
    }
}

/// Execution Contract module
pub mod contract {
    use serde::{Deserialize, Serialize};
    use chrono::{DateTime, Utc};
    use crate::ir::IRStep;
    
    /// Execution Contract
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ExecutionContract {
        pub ir_id: String,
        pub run_id: String,
        pub target: TargetSpec,
        pub steps: Vec<ExecStep>,
        pub policy: ExecPolicy,
        pub context: serde_json::Value,
        pub created_at: DateTime<Utc>,
    }
    
    /// Execution Step (resolved from IR)
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ExecStep {
        pub step_id: String,
        pub collector: String,
        pub action: String,
        pub params: serde_json::Value,
        pub output_vars: Vec<String>,
        pub timeout_ms: u64,
        pub retry: RetryPolicy,
    }
    
    /// Target Specification
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TargetSpec {
        pub target_type: String,
        pub identifier: String,
        pub os: String,
        pub arch: String,
    }
    
    /// Retry Policy
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RetryPolicy {
        pub max_attempts: u32,
        pub backoff_ms: u64,
    }
    
    /// Execution Policy
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ExecPolicy {
        pub parallelism: u32,
        pub continue_on_error: bool,
        pub evidence_root: String,
        pub hash_algorithm: String,
    }
    
    /// Contract build error
    #[derive(Debug, thiserror::Error)]
    pub enum ContractError {
        #[error("Unsupported OS: {0}")]
        UnsupportedOS(String),
        
        #[error("Collector not found: {0}")]
        CollectorNotFound(String),
        
        #[error("Action not supported: {0}")]
        ActionNotSupported(String),
        
        #[error("Parameter validation failed: {0}")]
        ParamValidationFailed(String),
        
        #[error("Capability denied: {0}")]
        CapabilityDenied(String),
        
        #[error("Invalid timeout: {0}")]
        InvalidTimeout(String),
        
        #[error("Invalid evidence root: {0}")]
        InvalidEvidenceRoot(String),
        
        #[error("Cyclic dependency: {0}")]
        CyclicDependency(String),
        
        #[error("Hash algorithm unsupported: {0}")]
        HashAlgoUnsupported(String),
    }
    
    /// Build execution contract from IR
    pub fn build_contract(
        ir: &crate::ir::IrModule,
        target: TargetSpec,
        policy: ExecPolicy,
    ) -> Result<ExecutionContract, ContractError> {
        // Validate target OS
        validate_target(&target, ir)?;
        
        // Resolve collectors for each step
        let mut exec_steps = Vec::new();
        for step in &ir.steps {
            let exec_step = resolve_step(step, &target, &policy)?;
            exec_steps.push(exec_step);
        }
        
        // Validate policy bounds
        validate_policy(&policy)?;
        
        // Validate dependencies
        validate_dependencies(&ir.steps)?;
        
        Ok(ExecutionContract {
            ir_id: ir.id.clone(),
            run_id: ulid::Ulid::new().to_string(),
            target,
            steps: exec_steps,
            policy,
            context: serde_json::json!({}),
            created_at: chrono::Utc::now(),
        })
    }
    
    fn validate_target(target: &TargetSpec, ir: &crate::ir::IrModule) -> Result<(), ContractError> {
        // Check if target OS has collectors for all steps
        for step in &ir.steps {
            if !collector_supports_os(&step.collector, &target.os) {
                return Err(ContractError::UnsupportedOS(format!(
                    "Collector {} does not support OS: {}", step.collector, target.os
                )));
            }
        }
        Ok(())
    }
    
    fn collector_supports_os(collector: &str, os: &str) -> bool {
        match os {
            "windows" => collector.ends_with("_windows") || collector == "correlator" || collector == "exporter" || collector == "verifier",
            "linux" => collector.ends_with("_linux") || collector == "correlator" || collector == "exporter" || collector == "verifier",
            _ => false,
        }
    }
    
    fn resolve_step(step: &crate::ir::IRStep, target: &TargetSpec, policy: &ExecPolicy) -> Result<ExecStep, ContractError> {
        // Validate collector exists and supports action
        // In a real implementation, this would check against collector registry
        if step.collector.is_empty() {
            return Err(ContractError::CollectorNotFound(step.collector.clone()));
        }
        
        // Apply policy defaults
        let timeout_ms = step.condition.as_ref()
            .and_then(|c| parse_timeout(c))
            .unwrap_or(30000)
            .min(300000)
            .max(1000);
        
        Ok(ExecStep {
            step_id: step.id.clone(),
            collector: step.collector.clone(),
            action: step.action.clone(),
            params: step.params.clone(),
            output_vars: step.binds.clone(),
            timeout_ms,
            retry: RetryPolicy {
                max_attempts: 1,
                backoff_ms: 1000,
            },
        })
    }
    
    fn parse_timeout(condition: &str) -> Option<u64> {
        // Parse timeout from condition string
        // Simplified for now
        None
    }
    
    fn validate_policy(policy: &ExecPolicy) -> Result<(), ContractError> {
        if policy.parallelism < 1 || policy.parallelism > 16 {
            return Err(ContractError::InvalidTimeout("Parallelism must be 1-16".to_string()));
        }
        if policy.evidence_root.is_empty() {
            return Err(ContractError::InvalidEvidenceRoot("Evidence root must be set".to_string()));
        }
        if policy.hash_algorithm != "blake3" && policy.hash_algorithm != "sha256" {
            return Err(ContractError::HashAlgoUnsupported(policy.hash_algorithm.clone()));
        }
        Ok(())
    }
    
    fn validate_dependencies(steps: &[crate::ir::IRStep]) -> Result<(), ContractError> {
        let mut step_ids = std::collections::HashSet::new();
        for step in steps {
            if !step_ids.insert(step.id.clone()) {
                return Err(ContractError::CyclicDependency("Duplicate step ID".to_string()));
            }
        }
        
        for step in steps {
            for dep in &step.depends_on {
                if !step_ids.contains(dep) {
                    return Err(ContractError::MissingDependency(dep.clone()));
                }
            }
        }
        
        // Check for cycles using topological sort
        // Simplified - full implementation would use petgraph
        Ok(())
    }
}

/// Policy Engine module
pub mod policy {
    use crate::contract::{ExecPolicy, TargetSpec};
    use crate::collector::CollectorManifest;
    use std::collections::HashMap;
    
    /// Policy Engine
    pub struct PolicyEngine {
        collector_manifests: HashMap<String, CollectorManifest>,
    }
    
    impl PolicyEngine {
        pub fn new() -> Self {
            Self {
                collector_manifests: HashMap::new(),
            }
        }
        
        pub fn register_collector(&mut self, manifest: CollectorManifest) {
            self.collector_manifests.insert(manifest.name.clone(), manifest);
        }
        
        pub fn validate_contract(
            &self,
            contract: &crate::contract::ExecutionContract,
        ) -> Result<(), crate::contract::ContractError> {
            // Validate each step
            for step in &contract.steps {
                self.validate_step(step, &contract.target)?;
            }
            
            // Validate policy bounds
            if contract.policy.parallelism < 1 || contract.policy.parallelism > 16 {
                return Err(crate::contract::ContractError::InvalidTimeout(
                    "Parallelism must be 1-16".to_string()
                ));
            }
            
            if contract.policy.hash_algorithm != "blake3" && contract.policy.hash_algorithm != "sha256" {
                return Err(crate::contract::ContractError::HashAlgoUnsupported(
                    contract.policy.hash_algorithm.clone()
                ));
            }
            
            Ok(())
        }
        
        fn validate_step(&self, step: &crate::contract::ExecStep, target: &crate::contract::TargetSpec) -> Result<(), crate::contract::ContractError> {
            // Check collector exists
            let manifest = self.collector_manifests.get(&step.collector)
                .ok_or(crate::contract::ContractError::CollectorNotFound(step.collector.clone()))?;
            
            // Check OS compatibility
            if !manifest.supported_os.contains(&target.os) {
                return Err(crate::contract::ContractError::UnsupportedOS(format!(
                    "Collector {} does not support OS: {}", step.collector, target.os
                )));
            }
            
            // Check action supported
            if !manifest.actions.contains_key(&step.action) {
                return Err(crate::contract::ContractError::ActionNotSupported(step.action.clone()));
            }
            
            // Validate parameters against schema
            let action_manifest = &manifest.actions[&step.action];
            // In a real implementation, validate JSON against schema
            
            // Check capabilities
            let required_caps = manifest.actions[&step.action].capabilities.clone();
            let available_caps = manifest.capabilities.clone();
            
            for cap in required_caps {
                if !available_caps.contains(&cap) {
                    return Err(crate::contract::ContractError::CapabilityDenied(
                        format!("Missing capability: {}", cap)
                    ));
                }
            }
            
            // Check timeout bounds
            if step.timeout_ms < 1000 || step.timeout_ms > 300000 {
                return Err(crate::contract::ContractError::InvalidTimeout(
                    "Timeout must be 1000-300000 ms".to_string()
                ));
            }
            
            Ok(())
        }
    }
}

/// Storage module
pub mod storage {
    /// Storage placeholder
    pub struct Storage;
}

/// Target specification module
pub mod target {
    use crate::contract::TargetSpec;
    use serde::{Deserialize, Serialize};
    
    impl TargetSpec {
        pub fn live(identifier: impl Into<String>, os: impl Into<String>) -> Self {
            Self {
                target_type: "live".to_string(),
                identifier: identifier.into(),
                os: os.into(),
                arch: "x86_64".to_string(),
            }
        }
        
        pub fn dead(identifier: impl Into<String>, os: impl Into<String>) -> Self {
            Self {
                target_type: "dead".to_string(),
                identifier: identifier.into(),
                os: os.into(),
                arch: "x86_64".to_string(),
            }
        }
        
        pub fn image(path: impl Into<String>, os: impl Into<String>) -> Self {
            Self {
                target_type: "image".to_string(),
                identifier: path.into(),
                os: os.into(),
                arch: "x86_64".to_string(),
            }
        }
    }
}

/// Capability module
pub mod capability {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub enum Capability {
        ProcessEnumerate,
        ProcessGet,
        ProcessTree,
        FileEnumerate,
        FileHash,
        FileCollect,
        RegistryEnumerate,
        RegistryGet,
        RegistryMonitor,
        LogQuery,
        LogExport,
        LogTail,
        NetworkConnections,
        NetworkListen,
        NetworkCapture,
        NetworkResolve,
    }
}

/// Collector module
pub mod collector {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    
    /// Collector Manifest
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CollectorManifest {
        pub name: String,
        pub version: String,
        pub supported_os: Vec<String>,
        pub capabilities: Vec<String>,
        pub actions: HashMap<String, ActionManifest>,
        pub public_key: String,
    }
    
    /// Action Manifest
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ActionManifest {
        pub capabilities: Vec<String>,
        pub params_schema: serde_json::Value,
        pub description: String,
    }
    
    /// Built-in collector manifests
    pub fn builtin_manifests() -> Vec<CollectorManifest> {
        vec![
            CollectorManifest {
                name: "proc_windows".to_string(),
                version: "0.1.0".to_string(),
                supported_os: vec!["windows".to_string()],
                capabilities: vec!["ProcessEnumerate".to_string(), "ProcessGet".to_string(), "ProcessTree".to_string()],
                actions: {
                    let mut m = std::collections::HashMap::new();
                    m.insert("list".to_string(), crate::collector::ActionManifest {
                        capabilities: vec!["ProcessEnumerate".to_string()],
                        params_schema: serde_json::json!({ "type": "object", "properties": { "filter": { "type": "object" } } }),
                        description: "List processes".to_string(),
                    });
                    m.insert("get".to_string(), crate::collector::ActionManifest {
                        capabilities: vec!["ProcessGet".to_string()],
                        params_schema: serde_json::json!({ "type": "object", "properties": { "pid": { "type": "integer" } } }),
                        description: "Get process by PID".to_string(),
                    });
                    m.insert("tree".to_string(), crate::collector::ActionManifest {
                        capabilities: vec!["ProcessTree".to_string()],
                        params_schema: serde_json::json!({ "type": "object", "properties": { "root_pid": { "type": "integer" } } }),
                        description: "Build process tree".to_string(),
                    });
                    m
                },
                public_key: "".to_string(),
            },
            // Add more manifests...
        ]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stub_compiles() {
        assert!(true);
    }
}