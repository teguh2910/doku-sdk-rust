use crate::client::DokuClient;
use crate::error::Result;
use crate::models::{
    AkulakuCancelRequest, AkulakuOrderRequest, AkulakuRefundRequest, PaylaterResponse,
};

pub struct PaylaterService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> PaylaterService<'a> {
    pub async fn akulaku_generate_order(
        &self,
        request: &AkulakuOrderRequest,
    ) -> Result<PaylaterResponse> {
        let path = "/akulaku-peer-to-peer/v2/generate-order";
        self.client.post_direct(path, request).await
    }

    pub async fn akulaku_cancel(&self, request: &AkulakuCancelRequest) -> Result<PaylaterResponse> {
        let path = "/akulaku-peer-to-peer/v2/cancel";
        self.client.post_direct(path, request).await
    }

    pub async fn akulaku_refund(&self, request: &AkulakuRefundRequest) -> Result<PaylaterResponse> {
        let path = "/akulaku-peer-to-peer/v2/refund";
        self.client.post_direct(path, request).await
    }
}
