use super::common::{Customer, Order};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCPaymentPageRequest {
    pub order: Order,
    pub customer: Customer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_configuration: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCPaymentPageResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCPaymentPageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card_payment_page: Option<CCPaymentPageResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(flatten)]
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCRefundRequest {
    pub order: Order,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<RefundDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCRefundResponse {
    #[serde(flatten)]
    pub raw: serde_json::Value,
}
