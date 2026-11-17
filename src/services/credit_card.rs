use crate::client::DokuClient;
use crate::error::Result;
use crate::models::{
    CCPaymentPageRequest, CCPaymentPageResponse, CCRefundRequest, CCRefundResponse,
};

pub struct CreditCardService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> CreditCardService<'a> {
    pub async fn generate_payment_page(
        &self,
        request: &CCPaymentPageRequest,
    ) -> Result<CCPaymentPageResponse> {
        let path = "/credit-card/v1/payment-page";
        self.client.post_direct(path, request).await
    }

    pub async fn refund(&self, request: &CCRefundRequest) -> Result<CCRefundResponse> {
        let path = "/credit-card/v1/cancellation/credit-card/refund";
        self.client.post_direct(path, request).await
    }
}
