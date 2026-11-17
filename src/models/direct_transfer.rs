use super::common::{Customer, Order};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectTransferOrderRequest {
    pub order: Order,
    pub customer: Customer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_transfer_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectTransferResponse {
    #[serde(flatten)]
    pub raw: serde_json::Value,
}
