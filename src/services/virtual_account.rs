use crate::client::DokuClient;
use crate::error::Result;
use crate::models::{
    GeneratePaymentCodeRequest, PaymentCodeResponse, UpdatePaymentCodeRequest, VaChannel,
};

pub struct VirtualAccountService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> VirtualAccountService<'a> {
    pub async fn generate_payment_code(
        &self,
        channel: VaChannel,
        request: &GeneratePaymentCodeRequest,
    ) -> Result<PaymentCodeResponse> {
        let path = channel.endpoint_path();
        self.client.post_direct(path, request).await
    }

    pub async fn update_payment_code(
        &self,
        channel: VaChannel,
        request: &UpdatePaymentCodeRequest,
    ) -> Result<PaymentCodeResponse> {
        let path = channel.endpoint_path();
        self.client.patch_direct(path, request).await
    }
}
