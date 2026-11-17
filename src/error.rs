use thiserror::Error;

#[derive(Error, Debug)]
pub enum DokuError {
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Cryptographic or signature error: {0}")]
    Signature(String),

    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization / Deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("DOKU API error (status {status}): {message} (code: {code:?})")]
    Api {
        status: u16,
        code: Option<String>,
        message: String,
        raw_body: String,
    },

    #[error("Webhook signature verification failed: {0}")]
    WebhookVerificationFailed(String),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
}

pub type Result<T> = std::result::Result<T, DokuError>;
