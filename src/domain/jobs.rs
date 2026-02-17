use super::enums::{AlertType, Severity};
use serde::{Deserialize, Serialize};

/// Order job - triggered when a new order is created
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderJob {
    pub event_id: String,
    pub device_uuid: String,
}

/// Alert job - sends alerts based on various events
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlertJob {
    pub event_id: String,
    pub device_uuid: String,
    pub alert_type: AlertType,
    pub message: String,
    pub severity: Severity,
}

/// Email job - sends email notifications
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailJob {
    pub to: String,
    pub subject: String,
    pub body: String,
}
