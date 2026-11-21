use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer {
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

pub fn deserialize_string_or_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val = serde_json::Value::deserialize(deserializer)?;
    match val {
        serde_json::Value::Number(num) => num
            .as_u64()
            .ok_or_else(|| serde::de::Error::custom("invalid u64 number")),
        serde_json::Value::String(s) => s.parse::<u64>().map_err(serde::de::Error::custom),
        _ => Err(serde::de::Error::custom("expected number or string")),
    }
}

pub fn deserialize_opt_string_or_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<serde_json::Value>::deserialize(deserializer)?;
    match opt {
        Some(serde_json::Value::Number(num)) => num
            .as_u64()
            .ok_or_else(|| serde::de::Error::custom("invalid u64 number"))
            .map(Some),
        Some(serde_json::Value::String(s)) => {
            s.parse::<u64>().map_err(serde::de::Error::custom).map(Some)
        }
        Some(serde_json::Value::Null) | None => Ok(None),
        _ => Err(serde::de::Error::custom("expected number or string")),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub name: String,
    #[serde(deserialize_with = "deserialize_string_or_u64")]
    pub price: u64,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Order {
    pub invoice_number: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_string_or_u64"
    )]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_redirect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<LineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapAmount {
    pub value: String,
    pub currency: String,
}

impl SnapAmount {
    pub fn idr(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            currency: "IDR".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdditionalInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountReference>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountReference {
    pub id: String,
}
