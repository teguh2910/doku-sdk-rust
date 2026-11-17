use std::sync::Arc;
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

use crate::config::{DokuConfig, DokuConfigBuilder};
use crate::crypto::{
    generate_digest, generate_direct_signature, generate_snap_asymmetric_signature,
    generate_snap_symmetric_signature,
};
use crate::error::{DokuError, Result};
use crate::services::*;
use crate::webhook::WebhookVerifier;

#[derive(Clone)]
pub struct DokuClient {
    pub(crate) config: Arc<DokuConfig>,
    pub(crate) http: reqwest::Client,
}

#[derive(Default)]
pub struct DokuClientBuilder {
    config_builder: DokuConfigBuilder,
}

impl DokuClientBuilder {
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.client_id(client_id);
        self
    }

    pub fn secret_key(mut self, secret_key: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.secret_key(secret_key);
        self
    }

    pub fn private_key(mut self, private_key: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.private_key(private_key);
        self
    }

    pub fn environment(mut self, environment: crate::config::Environment) -> Self {
        self.config_builder = self.config_builder.environment(environment);
        self
    }

    pub fn sandbox(mut self) -> Self {
        self.config_builder = self.config_builder.sandbox();
        self
    }

    pub fn production(mut self) -> Self {
        self.config_builder = self.config_builder.production();
        self
    }

    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config_builder = self.config_builder.timeout(timeout);
        self
    }

    pub fn build(self) -> Result<DokuClient> {
        let config = self.config_builder.build()?;
        Ok(DokuClient::new(config))
    }
}

impl DokuClient {
    pub fn new(config: DokuConfig) -> Self {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .unwrap_or_default();
        Self {
            config: Arc::new(config),
            http,
        }
    }

    pub fn builder() -> DokuClientBuilder {
        DokuClientBuilder::default()
    }

    pub fn config(&self) -> &DokuConfig {
        &self.config
    }

    // --- Service Accessors ---

    pub fn checkout(&self) -> CheckoutService<'_> {
        CheckoutService { client: self }
    }

    pub fn virtual_account(&self) -> VirtualAccountService<'_> {
        VirtualAccountService { client: self }
    }

    pub fn credit_card(&self) -> CreditCardService<'_> {
        CreditCardService { client: self }
    }

    pub fn emoney(&self) -> EmoneyService<'_> {
        EmoneyService { client: self }
    }

    pub fn o2o(&self) -> O2OService<'_> {
        O2OService { client: self }
    }

    pub fn paylater(&self) -> PaylaterService<'_> {
        PaylaterService { client: self }
    }

    pub fn direct_debit(&self) -> DirectDebitService<'_> {
        DirectDebitService { client: self }
    }

    pub fn direct_transfer(&self) -> DirectTransferService<'_> {
        DirectTransferService { client: self }
    }

    pub fn snap(&self) -> SnapService<'_> {
        SnapService { client: self }
    }

    pub fn sub_account(&self) -> SubAccountService<'_> {
        SubAccountService { client: self }
    }

    pub fn orders(&self) -> OrdersService<'_> {
        OrdersService { client: self }
    }

    pub fn webhook(&self) -> WebhookVerifier<'_> {
        WebhookVerifier::new(&self.config.secret_key)
    }

    // --- Internal HTTP Dispatchers for DOKU Direct / Non-SNAP ---

    pub(crate) async fn post_direct<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Resp> {
        self.request_direct_with_body(reqwest::Method::POST, path, body)
            .await
    }

    pub(crate) async fn patch_direct<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Resp> {
        self.request_direct_with_body(reqwest::Method::PATCH, path, body)
            .await
    }

    pub(crate) async fn get_direct<Resp: DeserializeOwned>(&self, path: &str) -> Result<Resp> {
        let url = format!("{}{}", self.config.environment.base_url(), path);
        let request_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let signature = generate_direct_signature(
            &self.config.client_id,
            &request_id,
            &timestamp,
            path,
            None,
            &self.config.secret_key,
        )?;

        let mut headers = HeaderMap::new();
        headers.insert("Client-Id", HeaderValue::from_str(&self.config.client_id).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("Request-Id", HeaderValue::from_str(&request_id).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("Request-Timestamp", HeaderValue::from_str(&timestamp).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("Signature", HeaderValue::from_str(&signature).map_err(|e| DokuError::Configuration(e.to_string()))?);

        let res = self.http.get(&url).headers(headers).send().await?;
        self.handle_response(res).await
    }

    async fn request_direct_with_body<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: &Req,
    ) -> Result<Resp> {
        let url = format!("{}{}", self.config.environment.base_url(), path);
        let body_json = serde_json::to_string(body)?;
        let digest = generate_digest(&body_json);
        let request_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let signature = generate_direct_signature(
            &self.config.client_id,
            &request_id,
            &timestamp,
            path,
            Some(&digest),
            &self.config.secret_key,
        )?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("Client-Id", HeaderValue::from_str(&self.config.client_id).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("Request-Id", HeaderValue::from_str(&request_id).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("Request-Timestamp", HeaderValue::from_str(&timestamp).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("Signature", HeaderValue::from_str(&signature).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("Digest", HeaderValue::from_str(&digest).map_err(|e| DokuError::Configuration(e.to_string()))?);

        let res = self
            .http
            .request(method, &url)
            .headers(headers)
            .body(body_json)
            .send()
            .await?;

        self.handle_response(res).await
    }

    // --- Internal HTTP Dispatchers for DOKU SNAP (Bank Indonesia Open API) ---

    pub(crate) async fn post_snap_token<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Resp> {
        let private_key = self.config.private_key.as_deref().ok_or_else(|| {
            DokuError::MissingParameter(
                "private_key is required in DokuConfig for SNAP RSA token generation".to_string(),
            )
        })?;

        let url = format!("{}{}", self.config.environment.base_url(), path);
        let timestamp = Utc::now().to_rfc3339();
        let signature = generate_snap_asymmetric_signature(
            &self.config.client_id,
            &timestamp,
            private_key,
        )?;

        let body_json = serde_json::to_string(body)?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("X-CLIENT-KEY", HeaderValue::from_str(&self.config.client_id).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("X-TIMESTAMP", HeaderValue::from_str(&timestamp).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("X-SIGNATURE", HeaderValue::from_str(&signature).map_err(|e| DokuError::Configuration(e.to_string()))?);

        let res = self
            .http
            .post(&url)
            .headers(headers)
            .body(body_json)
            .send()
            .await?;

        self.handle_response(res).await
    }

    pub(crate) async fn post_snap<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
        access_token: &str,
    ) -> Result<Resp> {
        self.request_snap_with_body(reqwest::Method::POST, path, body, access_token)
            .await
    }

    pub(crate) async fn put_snap<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
        access_token: &str,
    ) -> Result<Resp> {
        self.request_snap_with_body(reqwest::Method::PUT, path, body, access_token)
            .await
    }

    pub(crate) async fn delete_snap<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
        access_token: &str,
    ) -> Result<Resp> {
        self.request_snap_with_body(reqwest::Method::DELETE, path, body, access_token)
            .await
    }

    async fn request_snap_with_body<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: &Req,
        access_token: &str,
    ) -> Result<Resp> {
        let url = format!("{}{}", self.config.environment.base_url(), path);
        let body_json = serde_json::to_string(body)?;
        let timestamp = Utc::now().to_rfc3339();
        let external_id = format!("RID_{}", Uuid::new_v4().simple());

        let signature = generate_snap_symmetric_signature(
            method.as_str(),
            path,
            access_token,
            &body_json,
            &timestamp,
            &self.config.secret_key,
        )?;

        let bearer_header = if access_token.starts_with("Bearer ") {
            access_token.to_string()
        } else {
            format!("Bearer {}", access_token)
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&bearer_header).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("X-TIMESTAMP", HeaderValue::from_str(&timestamp).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("X-SIGNATURE", HeaderValue::from_str(&signature).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("X-PARTNER-ID", HeaderValue::from_str(&self.config.client_id).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("X-EXTERNAL-ID", HeaderValue::from_str(&external_id).map_err(|e| DokuError::Configuration(e.to_string()))?);
        headers.insert("CHANNEL-ID", HeaderValue::from_static("H2H"));

        let res = self
            .http
            .request(method, &url)
            .headers(headers)
            .body(body_json)
            .send()
            .await?;

        self.handle_response(res).await
    }

    // --- Helper to parse HTTP responses ---

    async fn handle_response<Resp: DeserializeOwned>(&self, res: reqwest::Response) -> Result<Resp> {
        let status = res.status();
        let raw_body = res.text().await?;

        if !status.is_success() {
            let code = serde_json::from_str::<serde_json::Value>(&raw_body)
                .ok()
                .and_then(|v| {
                    v.get("error")
                        .and_then(|e| e.get("code"))
                        .or_else(|| v.get("responseCode"))
                        .and_then(|c| c.as_str())
                        .map(String::from)
                });

            return Err(DokuError::Api {
                status: status.as_u16(),
                code,
                message: format!("DOKU API returned status HTTP {}", status),
                raw_body,
            });
        }

        let parsed: Resp = serde_json::from_str(&raw_body)?;
        Ok(parsed)
    }
}
