use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutRequest {
    pub invoice_number: String,
    pub amount: u64,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub callback_url: Option<String>,
    pub auto_redirect: Option<bool>,
    pub payment_due_date: Option<u32>,
    pub customer_name: String,
    pub customer_email: String,
    pub customer_phone: Option<String>,
    pub customer_address: Option<String>,
    pub line_items: Option<Vec<LineItemDTO>>,
}

fn default_currency() -> String {
    "IDR".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItemDTO {
    pub name: String,
    pub price: u64,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutResponse {
    pub success: bool,
    pub invoice_number: String,
    pub payment_url: String,
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusResponse {
    pub success: bool,
    pub invoice_number: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyNotificationRequest {
    pub client_id: String,
    pub request_id: String,
    pub timestamp: String,
    pub signature: String,
    pub path: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyNotificationResponse {
    pub valid: bool,
    pub invoice_number: Option<String>,
    pub status: Option<String>,
    pub amount: Option<u64>,
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub sandbox: bool,
    pub mock: bool,
}
