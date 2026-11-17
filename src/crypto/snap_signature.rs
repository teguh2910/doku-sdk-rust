use base64::prelude::*;
use hmac::{Hmac, Mac};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::DecodePrivateKey;
use rsa::signature::{SignatureEncoding, Signer};
use rsa::RsaPrivateKey;
use sha2::{Digest, Sha256, Sha512};

/// Generates an asymmetric SHA256withRSA signature for SNAP B2B Token API.
/// `string_to_sign = client_id + "|" + x_timestamp`
pub fn generate_snap_asymmetric_signature(
    client_id: &str,
    timestamp: &str,
    private_key_pem: &str,
) -> Result<String, crate::error::DokuError> {
    let string_to_sign = format!("{}|{}", client_id, timestamp);

    // Try parsing as PKCS#8 first, then PKCS#1
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem)
        .or_else(|_| RsaPrivateKey::from_pkcs1_pem(private_key_pem))
        .map_err(|e| {
            crate::error::DokuError::Signature(format!(
                "Failed to parse RSA private key PEM (neither PKCS#8 nor PKCS#1): {}",
                e
            ))
        })?;

    let signing_key = SigningKey::<Sha256>::new(private_key);
    let signature = signing_key.sign(string_to_sign.as_bytes());
    let signature_bytes = signature.to_vec();

    Ok(BASE64_STANDARD.encode(signature_bytes))
}

/// Helper to minify a JSON string for SNAP hash calculation.
pub fn minify_json(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    match serde_json::from_str::<serde_json::Value>(trimmed) {
        Ok(v) => serde_json::to_string(&v).unwrap_or_else(|_| trimmed.to_string()),
        Err(_) => trimmed.to_string(),
    }
}

/// Generates a symmetric HMAC-SHA512 signature for SNAP Transaction APIs and Notifications.
/// `string_to_sign = HTTPMethod + ":" + EndpointUrl + ":" + AccessToken + ":" + Lowercase(Hex(SHA256(minify(body)))) + ":" + Timestamp`
pub fn generate_snap_symmetric_signature(
    method: &str,
    endpoint_path: &str,
    access_token: &str,
    body: &str,
    timestamp: &str,
    secret_key: &str,
) -> Result<String, crate::error::DokuError> {
    let minified = minify_json(body);
    let mut sha256_hasher = Sha256::new();
    sha256_hasher.update(minified.as_bytes());
    let body_sha256_hex = hex::encode(sha256_hasher.finalize()).to_lowercase();

    // Clean token: strip optional "Bearer " prefix if user passed it with prefix
    let clean_token = access_token.trim_start_matches("Bearer ").trim();

    let string_to_sign = format!(
        "{}:{}:{}:{}:{}",
        method.to_uppercase(),
        endpoint_path,
        clean_token,
        body_sha256_hex,
        timestamp
    );

    type HmacSha512 = Hmac<Sha512>;
    let mut mac = HmacSha512::new_from_slice(secret_key.as_bytes())
        .map_err(|e| crate::error::DokuError::Signature(format!("Invalid HMAC key: {}", e)))?;
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize().into_bytes();

    Ok(BASE64_STANDARD.encode(result))
}

/// Verifies a received SNAP HMAC-SHA512 signature.
pub fn verify_snap_symmetric_signature(
    method: &str,
    endpoint_path: &str,
    access_token: &str,
    body: &str,
    timestamp: &str,
    received_signature: &str,
    secret_key: &str,
) -> bool {
    match generate_snap_symmetric_signature(
        method,
        endpoint_path,
        access_token,
        body,
        timestamp,
        secret_key,
    ) {
        Ok(expected_sig) => {
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
