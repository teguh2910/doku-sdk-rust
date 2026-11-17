use super::common::SnapAmount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapBank {
    Bnc,
    Bni,
    Bri,
    Bsi,
    Btn,
    Danamon,
    Mandiri,
    Maybank,
    Permata,
    Sinarmas,
    Cimb,
    Other(String),
}

impl SnapBank {
    pub fn create_va_path(&self) -> &str {
        match self {
            SnapBank::Bnc | SnapBank::Bni | SnapBank::Cimb => {
                "/virtual-accounts/bi-snap-va/v1/transfer-va/create-va"
            }
            SnapBank::Bri => "/virtual-accounts/bi-snap-va/v1.1/transfer-va/create-va",
            SnapBank::Bsi => "/bi-snap-va/bsi/v1/transfer-va/create-va",
            SnapBank::Btn => "/bi-snap-va/btn/v1/transfer-va/create-va",
            SnapBank::Danamon => "/bi-snap-va/danamon/v1/transfer-va/create-va",
            SnapBank::Mandiri => "/bi-snap-va/mandiri/v1/transfer-va/create-va",
            SnapBank::Maybank => "/bi-snap-va/maybank/v1/transfer-va/create-va",
            SnapBank::Permata => "/bi-snap-va/permata/v1/transfer-va/create-va",
            SnapBank::Sinarmas => "/bi-snap-va/sinarmas/v1/transfer-va/create-va",
            SnapBank::Other(path) => path.as_str(),
        }
    }

    pub fn update_va_path(&self) -> &str {
        match self {
            SnapBank::Bnc | SnapBank::Bni | SnapBank::Cimb => {
                "/virtual-accounts/bi-snap-va/v1/transfer-va/update-va"
            }
            SnapBank::Bri => "/virtual-accounts/bi-snap-va/v1.1/transfer-va/update-va",
            SnapBank::Bsi => "/bi-snap-va/bsi/v1/transfer-va/update-va",
            SnapBank::Btn => "/bi-snap-va/btn/v1/transfer-va/update-va",
            SnapBank::Danamon => "/bi-snap-va/danamon/v1/transfer-va/update-va",
            SnapBank::Mandiri => "/bi-snap-va/mandiri/v1/transfer-va/update-va",
            SnapBank::Maybank => "/bi-snap-va/maybank/v1/transfer-va/update-va",
            SnapBank::Permata => "/bi-snap-va/permata/v1/transfer-va/update-va",
            SnapBank::Sinarmas => "/bi-snap-va/sinarmas/v1/transfer-va/update-va",
            SnapBank::Other(path) => path.as_str(),
        }
    }

    pub fn delete_va_path(&self) -> &str {
        match self {
            SnapBank::Bnc | SnapBank::Bni | SnapBank::Cimb => {
                "/virtual-accounts/bi-snap-va/v1/transfer-va/delete-va"
            }
            SnapBank::Bri => "/virtual-accounts/bi-snap-va/v1.1/transfer-va/delete-va",
            SnapBank::Bsi => "/bi-snap-va/bsi/v1/transfer-va/delete-va",
            SnapBank::Btn => "/bi-snap-va/btn/v1/transfer-va/delete-va",
            SnapBank::Danamon => "/bi-snap-va/danamon/v1/transfer-va/delete-va",
            SnapBank::Mandiri => "/bi-snap-va/mandiri/v1/transfer-va/delete-va",
            SnapBank::Maybank => "/bi-snap-va/maybank/v1/transfer-va/delete-va",
            SnapBank::Permata => "/bi-snap-va/permata/v1/transfer-va/delete-va",
            SnapBank::Sinarmas => "/bi-snap-va/sinarmas/v1/transfer-va/delete-va",
            SnapBank::Other(path) => path.as_str(),
        }
    }

    pub fn inquiry_va_path(&self) -> &str {
        match self {
            SnapBank::Bnc | SnapBank::Bni | SnapBank::Cimb => {
                "/virtual-accounts/bi-snap-va/v1/transfer-va/inquiry-va"
            }
            SnapBank::Bri => "/virtual-accounts/bi-snap-va/v1.1/transfer-va/inquiry-va",
            SnapBank::Bsi => "/bi-snap-va/bsi/v1/transfer-va/inquiry-va",
            SnapBank::Btn => "/bi-snap-va/btn/v1/transfer-va/inquiry-va",
            SnapBank::Danamon => "/bi-snap-va/danamon/v1/transfer-va/inquiry-va",
            SnapBank::Mandiri => "/bi-snap-va/mandiri/v1/transfer-va/inquiry-va",
            SnapBank::Maybank => "/bi-snap-va/maybank/v1/transfer-va/inquiry-va",
            SnapBank::Permata => "/bi-snap-va/permata/v1/transfer-va/inquiry-va",
            SnapBank::Sinarmas => "/bi-snap-va/sinarmas/v1/transfer-va/inquiry-va",
            SnapBank::Other(path) => path.as_str(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapTokenRequest {
    #[serde(rename = "grantType")]
    pub grant_type: String,
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<serde_json::Value>,
}

impl Default for SnapTokenRequest {
    fn default() -> Self {
        Self {
            grant_type: "client_credentials".to_string(),
            additional_info: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapB2b2cTokenRequest {
    #[serde(rename = "grantType")]
    pub grant_type: String,
    #[serde(rename = "authCode")]
    pub auth_code: String,
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapTokenResponse {
    #[serde(rename = "responseCode")]
    pub response_code: String,
    #[serde(rename = "responseMessage")]
    pub response_message: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "tokenType")]
    pub token_type: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapCreateVaRequest {
    #[serde(rename = "partnerServiceId")]
    pub partner_service_id: String,
    #[serde(rename = "customerNo", skip_serializing_if = "Option::is_none")]
    pub customer_no: Option<String>,
    #[serde(rename = "virtualAccountNo", skip_serializing_if = "Option::is_none")]
    pub virtual_account_no: Option<String>,
    #[serde(rename = "virtualAccountName")]
    pub virtual_account_name: String,
    #[serde(rename = "virtualAccountEmail", skip_serializing_if = "Option::is_none")]
    pub virtual_account_email: Option<String>,
    #[serde(rename = "virtualAccountPhone", skip_serializing_if = "Option::is_none")]
    pub virtual_account_phone: Option<String>,
    #[serde(rename = "trxId")]
    pub trx_id: String,
    #[serde(rename = "totalAmount")]
    pub total_amount: SnapAmount,
    #[serde(rename = "virtualAccountTrxType", skip_serializing_if = "Option::is_none")]
    pub virtual_account_trx_type: Option<String>,
    #[serde(rename = "expiredDate", skip_serializing_if = "Option::is_none")]
    pub expired_date: Option<String>,
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapVaData {
    #[serde(rename = "partnerServiceId", skip_serializing_if = "Option::is_none")]
    pub partner_service_id: Option<String>,
    #[serde(rename = "customerNo", skip_serializing_if = "Option::is_none")]
    pub customer_no: Option<String>,
    #[serde(rename = "virtualAccountNo", skip_serializing_if = "Option::is_none")]
    pub virtual_account_no: Option<String>,
    #[serde(rename = "virtualAccountName", skip_serializing_if = "Option::is_none")]
    pub virtual_account_name: Option<String>,
    #[serde(rename = "trxId", skip_serializing_if = "Option::is_none")]
    pub trx_id: Option<String>,
    #[serde(rename = "totalAmount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<SnapAmount>,
    #[serde(rename = "expiredDate", skip_serializing_if = "Option::is_none")]
    pub expired_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapVaResponse {
    #[serde(rename = "responseCode")]
    pub response_code: String,
    #[serde(rename = "responseMessage")]
    pub response_message: String,
    #[serde(rename = "virtualAccountData", skip_serializing_if = "Option::is_none")]
    pub virtual_account_data: Option<SnapVaData>,
    #[serde(flatten)]
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapDebitPaymentRequest {
    #[serde(rename = "partnerReferenceNo")]
    pub partner_reference_no: String,
    #[serde(rename = "validUpTo", skip_serializing_if = "Option::is_none")]
    pub valid_up_to: Option<String>,
    #[serde(rename = "pointOfInitiation", skip_serializing_if = "Option::is_none")]
    pub point_of_initiation: Option<String>,
    #[serde(rename = "amount")]
    pub amount: SnapAmount,
    #[serde(rename = "urlParam", skip_serializing_if = "Option::is_none")]
    pub url_param: Option<serde_json::Value>,
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapDebitPaymentResponse {
    #[serde(rename = "responseCode")]
    pub response_code: String,
    #[serde(rename = "responseMessage")]
    pub response_message: String,
    #[serde(rename = "referenceNo", skip_serializing_if = "Option::is_none")]
    pub reference_no: Option<String>,
    #[serde(rename = "partnerReferenceNo", skip_serializing_if = "Option::is_none")]
    pub partner_reference_no: Option<String>,
    #[serde(rename = "webRedirectUrl", skip_serializing_if = "Option::is_none")]
    pub web_redirect_url: Option<String>,
    #[serde(flatten)]
    pub raw: serde_json::Value,
}
