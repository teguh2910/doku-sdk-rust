use base64::prelude::*;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

/// Generates a Base64-encoded SHA-256 digest of the given JSON body string.
pub fn generate_digest(body: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    let hash = hasher.finalize();
    BASE64_STANDARD.encode(hash)
}

/// Constructs the canonical string-to-sign for DOKU Direct / Non-SNAP requests.
/// Note: There is NO trailing newline after the last component.
pub fn build_direct_component_string(
    client_id: &str,
    request_id: &str,
    timestamp: &str,
    target: &str,
    digest: Option<&str>,
) -> String {
    if let Some(digest_val) = digest {
        format!(
            "Client-Id:{}\nRequest-Id:{}\nRequest-Timestamp:{}\nRequest-Target:{}\nDigest:{}",
            client_id, request_id, timestamp, target, digest_val
        )
    } else {
        format!(
            "Client-Id:{}\nRequest-Id:{}\nRequest-Timestamp:{}\nRequest-Target:{}",
            client_id, request_id, timestamp, target
        )
    }
}

/// Generates the DOKU Direct HMAC-SHA256 signature formatted as `HMACSHA256=<base64>`.
pub fn generate_direct_signature(
    client_id: &str,
    request_id: &str,
    timestamp: &str,
    target: &str,
    digest: Option<&str>,
    secret_key: &str,
) -> Result<String, crate::error::DokuError> {
    let component_string = build_direct_component_string(client_id, request_id, timestamp, target, digest);

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .map_err(|e| crate::error::DokuError::Signature(format!("Invalid HMAC key: {}", e)))?;
    mac.update(component_string.as_bytes());
    let result = mac.finalize().into_bytes();

    let signature_base64 = BASE64_STANDARD.encode(result);
    Ok(format!("HMACSHA256={}", signature_base64))
}

/// Verifies a received DOKU Direct signature against the calculated signature.
pub fn verify_direct_signature(
    client_id: &str,
    request_id: &str,
    timestamp: &str,
    target: &str,
    digest: Option<&str>,
    received_signature: &str,
    secret_key: &str,
) -> bool {
    match generate_direct_signature(client_id, request_id, timestamp, target, digest, secret_key) {
        Ok(expected_sig) => {
            // Constant-time comparison or clean slice comparison
            let expected_bytes = expected_sig.as_bytes();
            let received_bytes = received_signature.as_bytes();
            if expected_bytes.len() != received_bytes.len() {
                return false;
            }
            let mut diff = 0u8;
            for (a, b) in expected_bytes.iter().zip(received_bytes.iter()) {
                diff |= a ^ b;
            }
            diff == 0
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digest_and_signature_generation() {
        let client_id = "MCH-0001";
        let request_id = "req-12345";
        let timestamp = "2026-11-17T08:00:00Z";
        let target = "/checkout/v1/payment";
        let body = r#"{"order":{"amount":10000}}"#;
        let secret_key = "secret_key_123";

        let digest = generate_digest(body);
        assert!(!digest.is_empty());

        let sig = generate_direct_signature(
            client_id,
            request_id,
            timestamp,
            target,
            Some(&digest),
            secret_key,
        )
        .unwrap();

        assert!(sig.starts_with("HMACSHA256="));
        assert!(verify_direct_signature(
            client_id,
            request_id,
            timestamp,
            target,
            Some(&digest),
            &sig,
            secret_key
        ));

        // Tampered signature must fail
        assert!(!verify_direct_signature(
            client_id,
            request_id,
            timestamp,
            target,
            Some(&digest),
            "HMACSHA256=invalid",
            secret_key
        ));
    }
}
