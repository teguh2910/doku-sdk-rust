use crate::client::DokuClient;
use crate::error::Result;
use crate::models::{
    SubAccountResponse, SubAccountV1CreateRequest, SubAccountV1PayoutRequest,
    SubAccountV1TransferRequest, SubAccountV2BalanceInquiryRequest, SubAccountV2DebitRequest,
    SubAccountV2RegisterRequest, SubAccountV2TransactionHistoryRequest, SubAccountV2TransferRequest,
};

pub struct SubAccountService<'a> {
    pub(crate) client: &'a DokuClient,
}

impl<'a> SubAccountService<'a> {
    // --- Sub Account V1 APIs ---

    pub async fn v1_get_balance(&self, account_id: &str) -> Result<SubAccountResponse> {
        let path = format!("/sac-merchant/v1/balances/{}", account_id);
        self.client.get_direct(&path).await
    }

    pub async fn v1_transfer(
        &self,
        request: &SubAccountV1TransferRequest,
    ) -> Result<SubAccountResponse> {
        let path = "/sac-merchant/v1/transfers";
        self.client.post_direct(path, request).await
    }

    pub async fn v1_payout(
        &self,
        request: &SubAccountV1PayoutRequest,
    ) -> Result<SubAccountResponse> {
        let path = "/sac-merchant/v1/payouts";
        self.client.post_direct(path, request).await
    }

    pub async fn v1_create_account(
        &self,
        request: &SubAccountV1CreateRequest,
    ) -> Result<SubAccountResponse> {
        let path = "/sac-merchant/v1/accounts";
        self.client.post_direct(path, request).await
    }

    // --- Sub Account V2 APIs ---

    pub async fn v2_balance_inquiry(
        &self,
        request: &SubAccountV2BalanceInquiryRequest,
    ) -> Result<SubAccountResponse> {
        let path = "/sub-account/v2.0/balance-inquiries";
        self.client.post_direct(path, request).await
    }

    pub async fn v2_transaction_history(
        &self,
        request: &SubAccountV2TransactionHistoryRequest,
    ) -> Result<SubAccountResponse> {
        let path = "/sub-account/v2.0/transaction-history-list";
        self.client.post_direct(path, request).await
    }

    pub async fn v2_debit(&self, request: &SubAccountV2DebitRequest) -> Result<SubAccountResponse> {
        let path = "/sub-account/v2.0/debit";
        self.client.post_direct(path, request).await
    }

    pub async fn v2_debit_cancel(
        &self,
        request: &SubAccountV2DebitRequest,
    ) -> Result<SubAccountResponse> {
        let path = "/sub-account/v2.0/debit/cancel";
        self.client.post_direct(path, request).await
    }

    pub async fn v2_transfer_payment(
        &self,
        request: &SubAccountV2TransferRequest,
    ) -> Result<SubAccountResponse> {
        let path = "/sub-account/v2.0/transfer-payment";
        self.client.post_direct(path, request).await
    }

    pub async fn v2_transfer_inquiry(
        &self,
        request: &SubAccountV2TransferRequest,
    ) -> Result<SubAccountResponse> {
        let path = "/sub-account/v2.0/transfer-inquiry";
        self.client.post_direct(path, request).await
    }

    pub async fn v2_register(
        &self,
        request: &SubAccountV2RegisterRequest,
    ) -> Result<SubAccountResponse> {
        let path = "/sub-account/v2.0/register";
        self.client.post_direct(path, request).await
    }
}
