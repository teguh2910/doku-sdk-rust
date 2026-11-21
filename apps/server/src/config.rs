#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub client_id: String,
    pub secret_key: String,
    pub is_production: bool,
    pub mock_mode: bool,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let bind_addr = std::env::var("BIND_ADDR")
            .or_else(|_| std::env::var("PORT").map(|p| format!("0.0.0.0:{}", p)))
            .unwrap_or_else(|_| "0.0.0.0:8080".to_string());

        let client_id = std::env::var("DOKU_CLIENT_ID")
            .unwrap_or_else(|_| "MCH-0001-DEMO".to_string());

        let secret_key = std::env::var("DOKU_SECRET_KEY")
            .unwrap_or_else(|_| "SK-DEMO-SECRET-KEY".to_string());

        let is_production = std::env::var("DOKU_IS_PRODUCTION")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let mock_mode = std::env::var("DOKU_MOCK")
            .map(|v| v == "true" || v == "1")
            .unwrap_or_else(|_| {
                client_id.starts_with("MCH-0001-DEMO")
                    || client_id.starts_with("sandbox_")
                    || secret_key.starts_with("SK-DEMO")
            });

        let cors_origin = std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "*".to_string());

        Ok(Self {
            bind_addr,
            client_id,
            secret_key,
            is_production,
            mock_mode,
            cors_origin,
        })
    }
}
