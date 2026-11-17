use super::common::{Customer, Order};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AkulakuOrderRequest {
    pub order: Order,
    pub customer: Customer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_to_peer_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AkulakuCancelRequest {
    pub order: Order,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AkulakuRefundRequest {
    pub order: Order,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaylaterResponse {
    #[serde(flatten)]
    pub raw: serde_json::Value,
}
