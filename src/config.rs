use std::time::Duration;

pub const DEFAULT_SANDBOX_URL: &str = "https://api-sandbox.doku.com";
pub const DEFAULT_PRODUCTION_URL: &str = "https://api.doku.com";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Sandbox,
    Production,
    Custom(String),
}

impl Environment {
    pub fn base_url(&self) -> &str {
        match self {
            Environment::Sandbox => DEFAULT_SANDBOX_URL,
            Environment::Production => DEFAULT_PRODUCTION_URL,
            Environment::Custom(url) => url.as_str(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DokuConfig {
    pub client_id: String,
    pub secret_key: String,
    /// Optional private key in PEM format (PKCS#1 or PKCS#8) for SNAP RSA signatures
    pub private_key: Option<String>,
    pub environment: Environment,
    pub timeout: Duration,
}

impl DokuConfig {
    pub fn builder() -> DokuConfigBuilder {
        DokuConfigBuilder::default()
    }

    pub fn new(
        client_id: impl Into<String>,
        secret_key: impl Into<String>,
        environment: Environment,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            secret_key: secret_key.into(),
            private_key: None,
            environment,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_private_key(mut self, private_key: impl Into<String>) -> Self {
        self.private_key = Some(private_key.into());
        self
    }
}

#[derive(Default)]
pub struct DokuConfigBuilder {
    client_id: Option<String>,
    secret_key: Option<String>,
    private_key: Option<String>,
    environment: Option<Environment>,
    timeout: Option<Duration>,
}

impl DokuConfigBuilder {
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn secret_key(mut self, secret_key: impl Into<String>) -> Self {
        self.secret_key = Some(secret_key.into());
        self
    }

    pub fn private_key(mut self, private_key: impl Into<String>) -> Self {
        self.private_key = Some(private_key.into());
        self
    }

    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    pub fn sandbox(self) -> Self {
        self.environment(Environment::Sandbox)
    }

    pub fn production(self) -> Self {
        self.environment(Environment::Production)
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<DokuConfig, crate::error::DokuError> {
        let client_id = self
            .client_id
            .ok_or_else(|| crate::error::DokuError::Configuration("client_id is required".into()))?;
        let secret_key = self
            .secret_key
            .ok_or_else(|| crate::error::DokuError::Configuration("secret_key is required".into()))?;

        Ok(DokuConfig {
            client_id,
            secret_key,
            private_key: self.private_key,
            environment: self.environment.unwrap_or(Environment::Sandbox),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
        })
    }
}
