pub mod config;
pub mod handlers;
pub mod models;
pub mod routes;

use std::collections::HashMap;
use std::sync::Arc;
use doku_sdk::DokuClient;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct MockOrderInfo {
    pub invoice_number: String,
    pub amount: u64,
    pub status: String,
    pub payment_url: String,
}

pub struct AppState {
    pub config: config::Config,
    pub client: DokuClient,
    pub mock_orders: Arc<RwLock<HashMap<String, MockOrderInfo>>>,
}

impl AppState {
    pub fn new(config: config::Config) -> Result<Self, Box<dyn std::error::Error>> {
        let builder = DokuClient::builder()
            .client_id(&config.client_id)
            .secret_key(&config.secret_key);

        let client = if config.is_production {
            builder.production().build()?
        } else {
            builder.sandbox().build()?
        };

        Ok(Self {
            config,
            client,
            mock_orders: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}
