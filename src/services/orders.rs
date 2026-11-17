use crate::client::DokuClient;
use crate::error::Result;
use crate::models::CheckStatusResponse;

pub struct OrdersService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> OrdersService<'a> {
    /// Checks transaction status by invoice number.
    /// Hits `GET /orders/v1/status/{invoice_number}`
    pub async fn check_status(&self, invoice_number: &str) -> Result<CheckStatusResponse> {
        let path = format!("/orders/v1/status/{}", invoice_number);
        self.client.get_direct(&path).await
    }
}
