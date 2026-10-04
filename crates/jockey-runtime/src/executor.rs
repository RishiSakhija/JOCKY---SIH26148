//! JOCKY Runtime — Executor

use crate::contract::ExecutionContract;
use crate::collector_registry::CollectorRegistry;
use crate::collectors::trait_def::{Collector, CollectorOutput, ExecutionContext, CollectorError};
use crate::evidence::Evidence;
use jockey_evidence::receipt::CollectorKeypair;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;
use petgraph::graph::DiGraphMap;
use petgraph::algo::toposort;

/// Executor
pub struct Executor {
    registry: Arc<CollectorRegistry>,
    keypairs: HashMap<String, Arc<jockey_evidence::receipt::CollectorKeypair>>,
}

impl Executor {
    pub fn new(registry: &CollectorRegistry) -> Self {
        let mut keypairs = HashMap::new();
        
        // Generate keypairs for each registered collector
        for collector in registry.get_all() {
            let keypair = Arc::new(jockey_evidence::receipt::CollectorKeypair::generate());
            keypairs.insert(collector.name().to_string(), keypair);
        }
        
        Self {
            registry: Arc::new(registry.clone()),
            keypairs,
        }
    }

    /// Execute a contract
    pub async fn execute(&self, contract: ExecutionContract) -> Result<ExecutionResult, RuntimeError> {
        let start_time = Instant::now();
        let mut evidence_collected = 0;
        let mut receipts_generated = 0;
        let mut provenance_records = 0;
        let mut errors = Vec::new();
        
        // Create evidence root directory
        std::fs::create_dir_all(&contract.policy.evidence_root)
            .map_err(|e| RuntimeError::StorageError(e.to_string()))?;

        // Plan execution (topological sort)
        let groups = self.build_execution_groups(&contract.steps)?;
        
        // Create execution context
        let context = crate::collectors::trait_def::ExecutionContext {
            run_id: contract.run_id.clone(),
            evidence_root: std::path::PathBuf::from(&contract.policy.evidence_root),
            target: contract.target.clone(),
            temp_dir: std::env::temp_dir().join(&contract.run_id),
            credentials: contract.context.clone(),
        };

        // Create semaphore for parallelism control
        let semaphore = Arc::new(tokio::sync::Semaphore::new(contract.policy.parallelism as usize));
        
        // Execute steps in topological order with parallelism
        for group in groups {
            let mut handles = Vec::new();
            
            for step in group {
                let permit = semaphore.clone().acquire_owned().await.unwrap();
                let collector = self.registry.get(&step.collector)
                    .ok_or_else(|| RuntimeError::CollectorNotFound(step.collector.clone()))?;
                
                let keypair = self.keypairs.get(&step.collector).cloned();
                let context = context.clone();
                let step_clone = step.clone();
                
                let handle = tokio::spawn(async move {
                    let _permit = permit; // Hold permit until task completes
                    
                    let collector_clone = collector.clone();
                    let keypair = keypair.clone();
                    
                    let result = collector_clone.execute(
                        &step_clone.action,
                        step_clone.params.clone(),
                        &context,
                    ).await;
                    
                    (step_clone.step_id, result, keypair)
                });
                
                handles.push(handle);
            }
            
            // Wait for all steps in group
            for handle in handles {
                match handle.await {
                    Ok((step_id, result, keypair)) => {
                        match result {
                            Ok(output) => {
                                // Generate receipts for evidence
                                for evidence in output.evidence {
                                    if let Some(keypair) = &keypair {
                                        // Generate receipt
                                        let _ = self.generate_receipt(&evidence, keypair.clone());
                                    }
                                }
                                
                                evidence_collected += output.evidence.len();
                                receipts_generated += output.evidence.len();
                            }
                            Err(e) => {
                                errors.push(format!("Step {} failed: {}", step_id, e));
                            }
                        }
                    Err(e) => {
                        errors.push(format!("Task join error: {}", e));
                    }
                }
            }
        }
        
        let duration_ms = start_time.elapsed().as_millis() as u64;
        
        let status = if errors.is_empty() {
            ExecutionStatus::Success
        } else if evidence_collected > 0 {
            ExecutionStatus::Partial
        } else {
            ExecutionStatus::Failed
        };
        
        Ok(ExecutionResult {
            run_id: contract.run_id.clone(),
            status,
            evidence_collected,
            receipts_generated,
            provenance_records: 0, // TODO
            findings: 0, // TODO
            duration_ms,
            errors,
        })
    }
    
    fn build_execution_groups(&self, steps: &[crate::contract::ExecStep]) -> Result<Vec<Vec<crate::contract::ExecStep>>, RuntimeError> {
        // Build dependency graph
        let mut graph = DiGraphMap::new();
        let mut node_map = std::collections::HashMap::new();
        
        for step in steps {
            let idx = graph.add_node(step.clone());
            node_map.insert(step.step_id.clone(), idx);
        }
        
        for step in steps {
            let from = node_map[&step.step_id];
            for dep_id in &step.depends_on {
                let to = node_map[dep_id];
                graph.add_edge(to, from, ());
            }
        }
        
        // Topological sort
        let sorted = toposort(&graph, None)
            .map_err(|_| RuntimeError::ContractValidation("Cyclic dependency detected".to_string()))?;
        
        // Group by level (steps that can run in parallel)
        let mut groups = Vec::new();
        let mut completed = std::collections::HashSet::new();
        
        for step_id in sorted {
            let step = steps.iter().find(|s| s.step_id == step_id).unwrap();
            
            if step.depends_on.iter().all(|d| completed.contains(d)) {
                if let Some(last_group) = groups.last_mut() {
                    if last_group.len() < 16 { // Max parallelism
                        last_group.push(step.clone());
                    } else {
                        groups.push(vec![step.clone()]);
                    }
                } else {
                    groups.push(vec![step.clone()]);
                }
                completed.insert(step_id);
            } else {
                // This shouldn't happen with proper topological sort
                groups.push(vec![step.clone()]);
                completed.insert(step_id);
            }
        }
        
        Ok(groups)
    }
    
    fn generate_receipt(&self, evidence: &crate::evidence::Evidence, keypair: Arc<jockey_evidence::receipt::CollectorKeypair>) -> Result<jockey_evidence::receipt::EvidenceReceipt, RuntimeError> {
        let host_info = jockey_evidence::receipt::HostInfo {
            hostname: "localhost".to_string(), // Simplified - would use hostname crate in real impl
            os: std::env::consts::OS.to_string(),
            kernel: None,
            collector_pid: std::process::id(),
        };
        
        let receipt = jockey_evidence::receipt::EvidenceReceipt::new(
            evidence,
            &evidence.collector,
            "0.1.0",
            &keypair,
            host_info,
        );
        
        Ok(receipt)
    }
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

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;
use petgraph::graph::DiGraphMap;
use petgraph::algo::toposort;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;