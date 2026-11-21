use std::sync::Arc;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

use doku_server::{config::Config, routes, AppState};

fn test_state() -> Arc<AppState> {
    let cfg = Config {
        bind_addr: "127.0.0.1:0".to_string(),
        client_id: "MCH-0001-DEMO".to_string(),
        secret_key: "SK-DEMO-SECRET-KEY".to_string(),
        is_production: false,
        mock_mode: true,
        cors_origin: "*".to_string(),
    };
    Arc::new(AppState::new(cfg).unwrap())
}

#[tokio::test]
async fn test_health_check() {
    let state = test_state();
    let app = routes::router(state);

    let req = Request::builder()
        .uri("/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
    assert_eq!(json["service"], "doku-server");
    assert_eq!(json["mock"], true);
}

#[tokio::test]
async fn test_checkout_and_status() {
    let state = test_state();
    let app = routes::router(state);

    // 1. Checkout
    let payload = json!({
        "invoice_number": "INV-TEST-001",
        "amount": 50000,
        "currency": "IDR",
        "callback_url": "http://localhost:3000/track?inv=INV-TEST-001",
        "auto_redirect": true,
        "customer_name": "Anton",
        "customer_email": "anton@example.com",
        "customer_phone": "08123456789",
        "line_items": [
            {
                "name": "50 Diamonds",
                "price": 50000,
                "quantity": 1
            }
        ]
    });

    let req = Request::builder()
        .uri("/api/v1/checkout")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let checkout_res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(checkout_res["success"], true);
    assert_eq!(checkout_res["invoice_number"], "INV-TEST-001");
    assert!(checkout_res["payment_url"].as_str().unwrap().contains("INV-TEST-001"));

    // 2. Status Inquiry
    let req = Request::builder()
        .uri("/api/v1/orders/INV-TEST-001/status")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let status_res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(status_res["success"], true);
    assert_eq!(status_res["invoice_number"], "INV-TEST-001");
    assert_eq!(status_res["status"], "PENDING");
}

#[tokio::test]
async fn test_verify_notification() {
    let state = test_state();
    let app = routes::router(state);

    let notif_body = json!({
        "order": {
            "invoice_number": "INV-NOTIF-001",
            "amount": 90000
        },
        "transaction": {
            "status": "SUCCESS",
            "date": "2026-11-21T10:00:00Z"
        }
    })
    .to_string();

    // Valid mock verification
    let payload = json!({
        "client_id": "MCH-0001-DEMO",
        "request_id": "req-12345",
        "timestamp": "2026-11-21T10:00:00Z",
        "signature": "test_valid_signature",
        "path": "/api/v1/payments/doku/notification",
        "body": notif_body
    });

    let req = Request::builder()
        .uri("/api/v1/notifications/verify")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let verify_res: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(verify_res["valid"], true);
    assert_eq!(verify_res["invoice_number"], "INV-NOTIF-001");
    assert_eq!(verify_res["status"], "SUCCESS");

    // Invalid signature
    let invalid_payload = json!({
        "client_id": "MCH-0001-DEMO",
        "request_id": "req-12345",
        "timestamp": "2026-11-21T10:00:00Z",
        "signature": "invalid_signature",
        "path": "/api/v1/payments/doku/notification",
        "body": notif_body
    });

    let req = Request::builder()
        .uri("/api/v1/notifications/verify")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&invalid_payload).unwrap()))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
