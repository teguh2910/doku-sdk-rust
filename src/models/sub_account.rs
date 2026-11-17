use serde::{Deserialize, Serialize};

// --- Sub Account V1 Models ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountV1CreateRequest {
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountV1TransferRequest {
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountV1PayoutRequest {
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

// --- Sub Account V2 Models ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountV2BalanceInquiryRequest {
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountV2TransactionHistoryRequest {
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountV2DebitRequest {
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountV2TransferRequest {
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountV2RegisterRequest {
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountResponse {
    #[serde(flatten)]
    pub raw: serde_json::Value,
}
