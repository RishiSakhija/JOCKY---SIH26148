//! JOCKY Core — Collector Module

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
                m.insert("list".to_string(), ActionManifest {
                    capabilities: vec!["ProcessEnumerate".to_string()],
                    params_schema: serde_json::json!({ "type": "object", "properties": { "filter": { "type": "object" } } }),
                    description: "List processes".to_string(),
                });
                m.insert("get".to_string(), ActionManifest {
                    capabilities: vec!["ProcessGet".to_string()],
                    params_schema: serde_json::json!({ "type": "object", "properties": { "pid": { "type": "integer" } } }),
                    description: "Get process by PID".to_string(),
                });
                m.insert("tree".to_string(), ActionManifest {
                    capabilities: vec!["ProcessTree".to_string()],
                    params_schema: serde_json::json!({ "type": "object", "properties": { "root_pid": { "type": "integer" } } }),
                    description: "Build process tree".to_string(),
                });
                m
            },
            public_key: "".to_string(),
        },
    ]
}