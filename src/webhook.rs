use crate::crypto::{
    generate_digest, verify_direct_signature, verify_snap_symmetric_signature,
};

pub struct WebhookVerifier<'a> {
    secret_key: &'a str,
}

impl<'a> WebhookVerifier<'a> {
    pub fn new(secret_key: &'a str) -> Self {
        Self { secret_key }
    }

    /// Verifies a DOKU Direct / Non-SNAP webhook notification.
    ///
    /// Requires:
    /// - `client_id`: From incoming `Client-Id` header
    /// - `request_id`: From incoming `Request-Id` header
    /// - `timestamp`: From incoming `Request-Timestamp` header
    /// - `target_path`: The URL path of your notification endpoint (e.g. `/api/payments/doku-notification`)
    /// - `raw_body`: Raw JSON payload received
    /// - `received_signature`: From incoming `Signature` header (e.g. `HMACSHA256=...`)
    pub fn verify_direct(
        &self,
        client_id: &str,
        request_id: &str,
        timestamp: &str,
        target_path: &str,
        raw_body: &str,
        received_signature: &str,
    ) -> bool {
        let digest = if raw_body.is_empty() {
            None
        } else {
            Some(generate_digest(raw_body))
        };

        verify_direct_signature(
            client_id,
            request_id,
            timestamp,
            target_path,
            digest.as_deref(),
            received_signature,
            self.secret_key,
        )
    }

    /// Verifies a DOKU SNAP webhook notification.
    ///
    /// Requires:
    /// - `method`: HTTP method (usually `"POST"`)
    /// - `target_path`: The URL path of your notification endpoint (e.g. `/bi-snap-va/v1/payment`)
    /// - `access_token`: Access token from `Authorization` header (`Bearer ...`)
    /// - `raw_body`: Raw JSON payload received
    /// - `timestamp`: From incoming `X-TIMESTAMP` header
    /// - `received_signature`: From incoming `X-SIGNATURE` header
    pub fn verify_snap(
        &self,
        method: &str,
        target_path: &str,
        access_token: &str,
        raw_body: &str,
        timestamp: &str,
        received_signature: &str,
    ) -> bool {
        verify_snap_symmetric_signature(
            method,
            target_path,
            access_token,
            raw_body,
            timestamp,
            received_signature,
            self.secret_key,
        )
    }
}

/// Standalone helper to verify a DOKU Direct webhook notification.
pub fn verify_direct_notification(
    client_id: &str,
    request_id: &str,
    timestamp: &str,
    target_path: &str,
    raw_body: &str,
    received_signature: &str,
    secret_key: &str,
) -> bool {
    WebhookVerifier::new(secret_key).verify_direct(
        client_id,
        request_id,
        timestamp,
        target_path,
        raw_body,
        received_signature,
    )
}

/// Standalone helper to verify a DOKU SNAP webhook notification.
pub fn verify_snap_notification(
    method: &str,
    target_path: &str,
    access_token: &str,
    raw_body: &str,
    timestamp: &str,
    received_signature: &str,
    secret_key: &str,
) -> bool {
    WebhookVerifier::new(secret_key).verify_snap(
        method,
        target_path,
        access_token,
        raw_body,
        timestamp,
        received_signature,
    )
}
