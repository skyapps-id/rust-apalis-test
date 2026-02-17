use serde::{Deserialize, Serialize};

/// Types of alerts that can be triggered
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AlertType {
    OtaTimeout,
    DeviceOffline,
    AuthenticationFailed,
    Other(String),
}

/// Severity levels for alerts
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}
