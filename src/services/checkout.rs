use crate::client::DokuClient;
use crate::error::Result;
use crate::models::{CheckoutRequest, CheckoutResponse};

pub struct CheckoutService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> CheckoutService<'a> {
    pub async fn initiate_payment(&self, request: &CheckoutRequest) -> Result<CheckoutResponse> {
        let path = "/checkout/v1/payment";
        self.client.post_direct(path, request).await
    }
}
