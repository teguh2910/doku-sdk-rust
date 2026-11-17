use super::common::Order;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientIdRef {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OvoInfo {
    pub ovo_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OvoPaymentRequest {
    pub client: ClientIdRef,
    pub order: Order,
    pub ovo_info: OvoInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OvoCancelRequest {
    pub client: ClientIdRef,
    pub order: Order,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopeePayInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopeePayOrderRequest {
    pub order: Order,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopee_pay_info: Option<ShopeePayInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmoneyResponse {
    #[serde(flatten)]
    pub raw: serde_json::Value,
}
