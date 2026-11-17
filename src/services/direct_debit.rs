use crate::client::DokuClient;
use crate::error::Result;
use crate::models::{
    BriCardTokenRequest, BriOtpRequest, BriPaymentRequest, DirectDebitResponse,
};

pub struct DirectDebitService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> DirectDebitService<'a> {
    pub async fn bri_create_token(
        &self,
        request: &BriCardTokenRequest,
    ) -> Result<DirectDebitResponse> {
        let path = "/direct-debit/v1/token";
        self.client.post_direct(path, request).await
    }

    pub async fn bri_validate_otp(&self, request: &BriOtpRequest) -> Result<DirectDebitResponse> {
        let path = "/direct-debit/v1/token/validate";
        self.client.post_direct(path, request).await
    }

    pub async fn bri_resend_otp(&self, request: &BriOtpRequest) -> Result<DirectDebitResponse> {
        let path = "/direct-debit/v1/token-otp";
        self.client.post_direct(path, request).await
    }

    pub async fn bri_get_tokens(&self) -> Result<DirectDebitResponse> {
        let path = "/direct-debit/v1/tokens";
        self.client.get_direct(path).await
    }

    pub async fn bri_payment(&self, request: &BriPaymentRequest) -> Result<DirectDebitResponse> {
        let path = "/direct-debit/v1/payment";
        self.client.post_direct(path, request).await
    }
}
