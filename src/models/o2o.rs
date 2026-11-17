use super::common::{Customer, Order};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OnlineToOfflineInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReceiptFooter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_message_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_message_2: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoreReceiptInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<ReceiptFooter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct O2OPaymentCodeRequest {
    pub order: Order,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_to_offline_info: Option<OnlineToOfflineInfo>,
    pub customer: Customer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alfa_info: Option<StoreReceiptInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indomaret_info: Option<StoreReceiptInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct O2OPaymentCodeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_to_offline_info: Option<serde_json::Value>,
    #[serde(flatten)]
    pub raw: serde_json::Value,
}
