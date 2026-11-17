use crate::client::DokuClient;
use crate::error::Result;
use crate::models::{DirectTransferOrderRequest, DirectTransferResponse};

pub struct DirectTransferService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> DirectTransferService<'a> {
    pub async fn bca_generate_amount(
        &self,
        request: &DirectTransferOrderRequest,
    ) -> Result<DirectTransferResponse> {
        let path = "/bca-direct-transfer/v1/order";
        self.client.post_direct(path, request).await
    }
}
