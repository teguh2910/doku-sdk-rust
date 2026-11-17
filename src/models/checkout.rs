use super::common::{AdditionalInfo, Customer, Order};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckoutPayment {
    /// Expiration time in minutes (e.g. 60)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub order: Order,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<CheckoutPayment>,
    pub customer: Customer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<AdditionalInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutPaymentResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<CheckoutPaymentResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<CheckoutResponseBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Vec<String>>,
    #[serde(flatten)]
    pub raw: serde_json::Value,
}
