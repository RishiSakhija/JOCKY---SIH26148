//! JOCKY Core — Target Specification Module

use crate::contract::TargetSpec;

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