use crate::client::DokuClient;
use crate::error::Result;
use crate::models::{EmoneyResponse, OvoCancelRequest, OvoPaymentRequest, ShopeePayOrderRequest};

pub struct EmoneyService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> EmoneyService<'a> {
    pub async fn ovo_payment(&self, request: &OvoPaymentRequest) -> Result<EmoneyResponse> {
        let path = "/ovo-emoney/v1/payment";
        self.client.post_direct(path, request).await
    }

    pub async fn ovo_cancel(&self, request: &OvoCancelRequest) -> Result<EmoneyResponse> {
        let path = "/ovo-emoney/v1/cancel";
        self.client.post_direct(path, request).await
    }

    pub async fn shopeepay_order(&self, request: &ShopeePayOrderRequest) -> Result<EmoneyResponse> {
        let path = "/shopeepay-emoney/v2/order";
        self.client.post_direct(path, request).await
    }
}
