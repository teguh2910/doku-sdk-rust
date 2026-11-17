use base64::Engine;
use doku_sdk::crypto::*;
use doku_sdk::webhook::*;

const TEST_PRIVATE_KEY_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCx1sG5T84ty8oA
W4IgQIOsR2hmWoI4xSn6TDrr/WsF5Z53vm35Hbj4lqvsRGEvqSzIExapGKe64TP3
dLeMvCfpcixHsm05Dzdxfv2Wd9XWSE3XsexNH8HGpSE56aNsmU1se4l96zkujJOs
iBfwwo4wDQcDp57o9T4tJOXdk827DA2zsQXBBTcaDZ6iqsixYUvJXa76SFUtyXC+
Sm9rk7n6bomSCoSuEs6FiKQ6rNSQSU4UQD4PpNPv+g6Aeem0MIBD+2kAOncXSzbv
eMxd4BLooypOJaMI/bZxfqkiLXUo3ll0kopjmvzN6KAxJgTEOok64EMjREhm9l9J
loBdXpUXAgMBAAECggEAYzoXl2VO/kCB8lccsRAuJ/cj6lA/7pscQ/s6uSgKCh1H
oTqQs/oAljvDq62vqF81jGESOMF3/Q7g5wapz43Am0HIMSz+QmMaJWqc5dLxboGj
ZWFyHspEXakPc/fH5rVW/NulKKwRmMZ5M0AwU3bKzUbHjiN9yC/mRFwdIdfxZ5Zx
BfKL8X9d2OmzGnR/76PFJrjFKnPVKLSQZyBvMJgk/INrJ5F2t7Twcmf9EnHScDWI
BtCd4EmIuejLMB4kC7lCcWNCOL9DBNL2ka9YeLZ34tjE4Itvowtz1hgdNcAkoFAt
wMRxUf9vVqkukqt/0c37UAsrwjXFIAfRkSKatV0WGQKBgQDWxuDPKwsCeY6VAY0f
aa/kKbSBgau2Sgsgx0yNvtCx2/gK1eKuHScXbCer8sXYVPO9nwNkynMEO5pec4vV
fhmleDrKH18kYXafNeTuXV4gotnm51OWOaQZkwIBnUV8BxnMlL9gicIn4bFNrtaW
P+MmHafacjScJr3li+DTAApWawKBgQDT+Ou6Mck7c2guAv2RDl9CH6o267Sr2fcU
29uKSHchp0qmrMcFfA27NMqC4zsYIw41wnIOrGXwh0jg1UHHqjswGoyO3ED4VCec
7AiHOd97As3y48Mvm5liYx5LeRRJML5zvBH14W490NUHXOsSNWpJ49EnoxkLbcS9
h9R30LfvBQKBgAuHdX3pc5fYapz5DlhHVhfLXVMuYMdSRjKmQCfDvHQvuKrsPFTY
X8bUGXUTfSdVuBfPt2DrtSze/jRfw3vcXwgQ4icDdBoB2VpGjFl3TZw24beRZ11D
DtoImCz6AkbcIrhmq7dE7dUyOlBPOFmsaEcdhGsngkd3lO4DijgJYoStAoGBALZ5
hWvO6f1sttVNK72Qkr5Yhe4rcGe0BjiFn3NmOG/XFLNw7w8woJ10r5SHSlrX6qaq
0gcBr+ukej9aNftM9xDM+g0NXDyqLUOwCRNn5ZN+cY5TNbapCxygWPb9r1HmTT1S
PF/fhUhfmtNw0MfnBa3AGVA23htxjjjeUjAmlACFAoGBAKxnvtLNDatmsuvxkKo4
zUvrDVc4lRCZBXZvY6HXnaK9BLeyJ16Hb8yz9Ig222gKr4PAJCW5DOJKbHqHuGhZ
AAjXo3wdH4AmODAh/kkysZlyQ2pY52r0C7xjsIeEGJbrwIBLWL3tEd3Hsjwnj6gd
MdU1l0PsqOEv0kYbuKHVaGmm
-----END PRIVATE KEY-----"#;

#[test]
fn test_direct_hmac_sha256_signature() {
    let client_id = "MCH-0001";
    let request_id = "f05e94b2-c7f7-45e3-9de6-4f7f628468f7";
    let timestamp = "2026-11-17T08:30:00Z";
    let target = "/checkout/v1/payment";
    let body = r#"{"order":{"invoice_number":"INV-12345","amount":10000}}"#;
    let secret = "secret_key_sample";

    let digest = generate_digest(body);
    let signature = generate_direct_signature(
        client_id,
        request_id,
        timestamp,
        target,
        Some(&digest),
        secret,
    )
    .expect("Signature generation should succeed");

    assert!(signature.starts_with("HMACSHA256="));

    let is_valid = verify_direct_signature(
        client_id,
        request_id,
        timestamp,
        target,
        Some(&digest),
        &signature,
        secret,
    );
    assert!(is_valid);

    // Tampered payload verification failure
    let tampered_digest =
        generate_digest(r#"{"order":{"invoice_number":"INV-12345","amount":99999}}"#);
    let is_tampered_valid = verify_direct_signature(
        client_id,
        request_id,
        timestamp,
        target,
        Some(&tampered_digest),
        &signature,
        secret,
    );
    assert!(!is_tampered_valid);
}

#[test]
fn test_direct_get_signature_without_digest() {
    let client_id = "MCH-0001";
    let request_id = "get-request-123";
    let timestamp = "2026-11-17T08:30:00Z";
    let target = "/orders/v1/status/INV-12345";
    let secret = "secret_key_sample";

    let signature =
        generate_direct_signature(client_id, request_id, timestamp, target, None, secret)
            .expect("GET signature generation should succeed");

    assert!(signature.starts_with("HMACSHA256="));

    let is_valid = verify_direct_signature(
        client_id,
        request_id,
        timestamp,
        target,
        None,
        &signature,
        secret,
    );
    assert!(is_valid);
}

#[test]
fn test_snap_asymmetric_rsa_signing() {
    let client_id = "MCH-SNAP-001";
    let timestamp = "2026-11-17T08:30:00+07:00";

    let signature =
        generate_snap_asymmetric_signature(client_id, timestamp, TEST_PRIVATE_KEY_PEM)
            .expect("SNAP RSA signature should succeed");

    assert!(!signature.is_empty());
    assert!(base64::prelude::BASE64_STANDARD.decode(&signature).is_ok());
}

#[test]
fn test_snap_symmetric_hmac_sha512_signing() {
    let method = "POST";
    let path = "/virtual-accounts/bi-snap-va/v1/transfer-va/create-va";
    let access_token = "eyJh...sample_token";
    let body = r#"{ "partnerServiceId": "02186", "trxId": "INV-1001", "totalAmount": { "value": "12500.00", "currency": "IDR" } }"#;
    let timestamp = "2026-11-17T08:30:00+07:00";
    let secret = "secret_key_snap";

    let signature =
        generate_snap_symmetric_signature(method, path, access_token, body, timestamp, secret)
            .expect("SNAP symmetric signature should succeed");

    assert!(!signature.is_empty());

    let is_valid = verify_snap_symmetric_signature(
        method,
        path,
        access_token,
        body,
        timestamp,
        &signature,
        secret,
    );
    assert!(is_valid);

    // Tampered body should fail verification
    let tampered_body = r#"{ "partnerServiceId": "02186", "trxId": "INV-9999", "totalAmount": { "value": "12500.00", "currency": "IDR" } }"#;
    let is_tampered_valid = verify_snap_symmetric_signature(
        method,
        path,
        access_token,
        tampered_body,
        timestamp,
        &signature,
        secret,
    );
    assert!(!is_tampered_valid);
}

#[test]
fn test_webhook_verifiers() {
    let secret = "merchant_secret_key";
    let verifier = WebhookVerifier::new(secret);

    // Direct notification
    let client_id = "MCH-001";
    let request_id = "req-999";
    let timestamp = "2026-11-17T08:30:00Z";
    let target = "/payments/notifications";
    let body = r#"{"order":{"invoice_number":"INV-01"},"transaction":{"status":"SUCCESS"}}"#;

    let digest = generate_digest(body);
    let sig = generate_direct_signature(
        client_id,
        request_id,
        timestamp,
        target,
        Some(&digest),
        secret,
    )
    .unwrap();

    assert!(verifier.verify_direct(client_id, request_id, timestamp, target, body, &sig));
    assert!(verify_direct_notification(
        client_id,
        request_id,
        timestamp,
        target,
        body,
        &sig,
        secret
    ));

    // SNAP notification
    let method = "POST";
    let snap_target = "/bi-snap-va/v1/payment";
    let access_token = "bearer_token_abc";
    let snap_timestamp = "2026-11-17T08:30:00+07:00";
    let snap_body =
        r#"{"virtualAccountNo":"021862040000016","paidAmount":{"value":"12500.00","currency":"IDR"}}"#;

    let snap_sig = generate_snap_symmetric_signature(
        method,
        snap_target,
        access_token,
        snap_body,
        snap_timestamp,
        secret,
    )
    .unwrap();

    assert!(verifier.verify_snap(
        method,
        snap_target,
        access_token,
        snap_body,
        snap_timestamp,
        &snap_sig
    ));
    assert!(verify_snap_notification(
        method,
        snap_target,
        access_token,
        snap_body,
        snap_timestamp,
        &snap_sig,
        secret
    ));
}
