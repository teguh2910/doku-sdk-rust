use doku_sdk::{DokuClient, DokuConfig, Environment};
use std::time::Duration;

#[test]
fn test_client_builder() {
    let config = DokuConfig::builder()
        .client_id("MCH-0001-TEST")
        .secret_key("SK-SECRET-TEST")
        .sandbox()
        .timeout(Duration::from_secs(15))
        .build()
        .expect("Config build should succeed");

    assert_eq!(config.client_id, "MCH-0001-TEST");
    assert_eq!(config.secret_key, "SK-SECRET-TEST");
    assert_eq!(config.environment, Environment::Sandbox);
    assert_eq!(config.environment.base_url(), "https://api-sandbox.doku.com");
    assert_eq!(config.timeout, Duration::from_secs(15));

    let client = DokuClient::new(config);
    assert_eq!(client.config().client_id, "MCH-0001-TEST");
    assert_eq!(client.config().secret_key, "SK-SECRET-TEST");
}

#[test]
fn test_client_builder_missing_credentials() {
    let res = DokuConfig::builder()
        .client_id("MCH-0001-TEST")
        // missing secret_key
        .build();

    assert!(res.is_err());
}
