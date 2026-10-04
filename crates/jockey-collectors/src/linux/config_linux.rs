//! Linux Config Collector — Stub Implementation

use async_trait::async_trait;
use crate::trait_def::{Collector, CollectorOutput, ExecutionContext, CollectorError, CollectorMetrics, ArtifactRef};
use serde_json::Value;

pub struct ConfigLinuxCollector;

#[async_trait]
impl Collector for ConfigLinuxCollector {
    fn name(&self) -> &'static str {
        "config_linux"
    }

    fn version(&self) -> &'static str {
        "0.1.0"
    }

    fn supported_actions(&self) -> Vec<&'static str> {
        vec!["enumerate", "get", "systemd", "journald"]
    }

    fn supported_os(&self) -> Vec<&'static str> {
        vec!["linux"]
    }

    async fn execute(
        &self,
        action: &str,
        _params: serde_json::Value,
        _context: &ExecutionContext,
    ) -> Result<CollectorOutput, crate::trait_def::CollectorError> {
        match action {
            _ => Err(crate::trait_def::CollectorError::UnsupportedAction(format!("config_linux action {} not implemented", action)))
        }
    }
}