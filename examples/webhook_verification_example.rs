use doku_sdk::webhook::verify_direct_notification;
use doku_sdk::WebhookVerifier;

fn main() {
    let secret_key = "MY_DOKU_SECRET_KEY";

    // --- Scenario 1: Non-SNAP Notification (e.g. from DOKU Checkout / VA) ---
    let client_id = "MCH-0001-DEMO";
    let request_id = "f05e94b2-c7f7-45e3-9de6-4f7f628468f7";
    let timestamp = "2026-11-17T08:30:00Z";
    let notification_path = "/api/v1/payments/doku/callback";
    let raw_json_body = r#"{
        "order": {
            "invoice_number": "INV-20261117-001",
            "amount": 90000
        },
        "transaction": {
            "status": "SUCCESS",
            "date": "2026-11-17T08:31:00Z"
        }
    }"#;

    // Suppose this is the header received from DOKU
    let digest = doku_sdk::crypto::generate_digest(raw_json_body);
    let signature = doku_sdk::crypto::generate_direct_signature(
        client_id,
        request_id,
        timestamp,
        notification_path,
        Some(&digest),
        secret_key,
    )
    .unwrap();

    // Verification using the WebhookVerifier
    let verifier = WebhookVerifier::new(secret_key);
    let is_valid = verifier.verify_direct(
        client_id,
        request_id,
        timestamp,
        notification_path,
        raw_json_body,
        &signature,
    );

    println!("Webhook verification result: {}", is_valid);
    assert!(is_valid);

    // Or using the standalone function:
    let is_valid_standalone = verify_direct_notification(
        client_id,
        request_id,
        timestamp,
        notification_path,
        raw_json_body,
        &signature,
        secret_key,
    );
    println!("Standalone verification: {}", is_valid_standalone);
    assert!(is_valid_standalone);

    // Parse notification body
    let notification: doku_sdk::models::DirectWebhookNotification =
        serde_json::from_str(raw_json_body).unwrap();
    println!(
        "Processed payment for invoice: {:?}",
        notification.order.map(|o| o.invoice_number)
    );
}
