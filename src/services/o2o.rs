use crate::client::DokuClient;
use crate::error::Result;
use crate::models::{O2OPaymentCodeRequest, O2OPaymentCodeResponse};

pub struct O2OService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> O2OService<'a> {
    pub async fn alfa_payment_code(
        &self,
        request: &O2OPaymentCodeRequest,
    ) -> Result<O2OPaymentCodeResponse> {
        let path = "/alfa-online-to-offline/v2/payment-code";
        self.client.post_direct(path, request).await
    }

    pub async fn indomaret_payment_code(
        &self,
        request: &O2OPaymentCodeRequest,
    ) -> Result<O2OPaymentCodeResponse> {
        let path = "/indomaret-online-to-offline/v2/payment-code";
        self.client.post_direct(path, request).await
    }
}
