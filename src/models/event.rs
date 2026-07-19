use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct EventSubscription {
    pub id: String,
    pub app_id: String,
    pub broadcaster_user_id: u64,
    pub event: String,
    pub version: u32,
    pub method: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeEvent {
    pub name: String,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcaster_user_id: Option<u64>,
    pub method: String,
    pub events: Vec<SubscribeEvent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubscribeResult {
    pub name: String,
    pub version: u32,
    pub subscription_id: Option<String>,
    pub error: Option<String>,
}