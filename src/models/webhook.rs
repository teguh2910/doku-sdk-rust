use super::common::{Customer, Order};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectPaymentResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectNotificationTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectWebhookNotification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<DirectNotificationTransaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<DirectPaymentResult>,
    #[serde(flatten)]
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapWebhookNotification {
    #[serde(flatten)]
    pub raw: serde_json::Value,
}
