use super::common::{AdditionalInfo, Customer, Order};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VaChannel {
    Mandiri,
    Bca,
    Bsi,
    Doku,
    Permata,
    Cimb,
    Bni,
}

impl VaChannel {
    pub fn endpoint_path(&self) -> &'static str {
        match self {
            VaChannel::Mandiri => "/mandiri-virtual-account/v2/payment-code",
            VaChannel::Bca => "/bca-virtual-account/v2/payment-code",
            VaChannel::Bsi => "/bsm-virtual-account/v2/payment-code",
            VaChannel::Doku => "/doku-virtual-account/v2/payment-code",
            VaChannel::Permata => "/permata-virtual-account/v2/payment-code",
            VaChannel::Cimb => "/cimb-virtual-account/v2/payment-code",
            VaChannel::Bni => "/bni-virtual-account/v2/payment-code",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VirtualAccountInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info3: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePaymentCodeRequest {
    pub order: Order,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_account_info: Option<VirtualAccountInfo>,
    pub customer: Customer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<AdditionalInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentCodeRequest {
    pub order: Order,
    pub virtual_account_info: VirtualAccountInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<AdditionalInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualAccountResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub how_to_pay_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub how_to_pay_api: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCodeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_account_info: Option<VirtualAccountResult>,
    #[serde(flatten)]
    pub raw: serde_json::Value,
}
