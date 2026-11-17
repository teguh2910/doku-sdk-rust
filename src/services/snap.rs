use crate::client::DokuClient;
use crate::error::Result;
use crate::models::{
    SnapB2b2cTokenRequest, SnapBank, SnapCreateVaRequest, SnapDebitPaymentRequest,
    SnapDebitPaymentResponse, SnapTokenRequest, SnapTokenResponse, SnapVaResponse,
};

pub struct SnapService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> SnapService<'a> {
    /// Obtains a B2B Access Token using Asymmetric RSA (SHA256withRSA) signature.
    pub async fn get_b2b_token(&self) -> Result<SnapTokenResponse> {
        let path = "/authorization/v1/access-token/b2b";
        let req = SnapTokenRequest::default();
        self.client.post_snap_token(path, &req).await
    }

    /// Obtains a B2B2C Access Token using authorization code.
    pub async fn get_b2b2c_token(
        &self,
        auth_code: impl Into<String>,
    ) -> Result<SnapTokenResponse> {
        let path = "/authorization/v1/access-token/b2b2c";
        let req = SnapB2b2cTokenRequest {
            grant_type: "authorization_code".to_string(),
            auth_code: auth_code.into(),
            additional_info: None,
        };
        self.client.post_snap_token(path, &req).await
    }

    /// Creates a SNAP Virtual Account for the specified bank.
    pub async fn create_va(
        &self,
        bank: SnapBank,
        request: &SnapCreateVaRequest,
        access_token: &str,
    ) -> Result<SnapVaResponse> {
        let path = bank.create_va_path();
        self.client.post_snap(path, request, access_token).await
    }

    /// Updates a SNAP Virtual Account.
    pub async fn update_va<T: serde::Serialize>(
        &self,
        bank: SnapBank,
        request: &T,
        access_token: &str,
    ) -> Result<SnapVaResponse> {
        let path = bank.update_va_path();
        self.client.put_snap(path, request, access_token).await
    }

    /// Deletes / Inactivates a SNAP Virtual Account.
    pub async fn delete_va<T: serde::Serialize>(
        &self,
        bank: SnapBank,
        request: &T,
        access_token: &str,
    ) -> Result<SnapVaResponse> {
        let path = bank.delete_va_path();
        self.client.delete_snap(path, request, access_token).await
    }

    /// Inquires a SNAP Virtual Account.
    pub async fn inquiry_va<T: serde::Serialize>(
        &self,
        bank: SnapBank,
        request: &T,
        access_token: &str,
    ) -> Result<SnapVaResponse> {
        let path = bank.inquiry_va_path();
        self.client.post_snap(path, request, access_token).await
    }

    /// SNAP Direct Debit & E-Wallet: Host-to-host payment
    pub async fn payment_host_to_host(
        &self,
        request: &SnapDebitPaymentRequest,
        access_token: &str,
    ) -> Result<SnapDebitPaymentResponse> {
        let path = "/direct-debit/core/v1/debit/payment-host-to-host";
        self.client.post_snap(path, request, access_token).await
    }

    /// SNAP Direct Debit & E-Wallet: Account Binding
    pub async fn account_binding<T: serde::Serialize>(
        &self,
        request: &T,
        access_token: &str,
    ) -> Result<serde_json::Value> {
        let path = "/direct-debit/core/v1/registration-account-binding";
        self.client.post_snap(path, request, access_token).await
    }

    /// SNAP Direct Debit & E-Wallet: Card Binding
    pub async fn card_binding<T: serde::Serialize>(
        &self,
        request: &T,
        access_token: &str,
    ) -> Result<serde_json::Value> {
        let path = "/direct-debit/core/v1/registration-card-bind";
        self.client.post_snap(path, request, access_token).await
    }

    /// SNAP Direct Debit & E-Wallet: OTP Verification
    pub async fn verify_otp<T: serde::Serialize>(
        &self,
        request: &T,
        access_token: &str,
    ) -> Result<serde_json::Value> {
        let path = "/direct-debit/core/v1/otp-verification";
        self.client.post_snap(path, request, access_token).await
    }

    /// SNAP Direct Debit & E-Wallet: Balance Inquiry
    pub async fn balance_inquiry<T: serde::Serialize>(
        &self,
        request: &T,
        access_token: &str,
    ) -> Result<serde_json::Value> {
        let path = "/direct-debit/core/v1/balance-inquiry";
        self.client.post_snap(path, request, access_token).await
    }

    /// SNAP Direct Debit & E-Wallet: Refund
    pub async fn refund<T: serde::Serialize>(
        &self,
        request: &T,
        access_token: &str,
    ) -> Result<serde_json::Value> {
        let path = "/direct-debit/core/v1/debit/refund";
        self.client.post_snap(path, request, access_token).await
    }
}
