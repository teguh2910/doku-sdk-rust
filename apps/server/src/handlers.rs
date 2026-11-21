use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use doku_sdk::models::{
    CheckoutPayment, CheckoutRequest, Customer, DirectWebhookNotification, LineItem, Order,
};
use crate::models::{
    CreateCheckoutRequest, CreateCheckoutResponse, HealthResponse, OrderStatusResponse,
    VerifyNotificationRequest, VerifyNotificationResponse,
};
use crate::AppState;

pub async fn health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "doku-server".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        sandbox: !state.config.is_production,
        mock: state.config.mock_mode,
    })
}

pub async fn checkout(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateCheckoutRequest>,
) -> impl IntoResponse {
    let invoice_number = req.invoice_number.clone();
    let amount = req.amount;

    if state.config.mock_mode {
        let payment_url = req
            .callback_url
            .clone()
            .unwrap_or_else(|| format!("/track?inv={}", invoice_number));
        let token = format!("demo-token-{}", invoice_number);

        {
            let mut orders = state.mock_orders.write().await;
            orders.insert(
                invoice_number.clone(),
                crate::MockOrderInfo {
                    invoice_number: invoice_number.clone(),
                    amount,
                    status: "PENDING".to_string(),
                    payment_url: payment_url.clone(),
                },
            );
        }

        return (
            StatusCode::OK,
            Json(CreateCheckoutResponse {
                success: true,
                invoice_number,
                payment_url,
                token: Some(token),
                message: Some("Created checkout session in mock/sandbox mode".to_string()),
                raw: None,
            }),
        );
    }

    let line_items = req.line_items.map(|items| {
        items
            .into_iter()
            .map(|item| LineItem {
                name: item.name,
                price: item.price,
                quantity: item.quantity,
            })
            .collect()
    });

    let checkout_req = CheckoutRequest {
        order: Order {
            invoice_number: invoice_number.clone(),
            amount: Some(amount),
            currency: Some(req.currency),
            callback_url: req.callback_url,
            auto_redirect: req.auto_redirect,
            line_items,
            session_id: None,
        },
        payment: Some(CheckoutPayment {
            payment_due_date: req.payment_due_date.or(Some(60)),
            payment_method_types: None,
        }),
        customer: {
            let customer_name = {
                let trimmed = req.customer_name.trim();
                if trimmed.is_empty() {
                    "Customer".to_string()
                } else {
                    trimmed.to_string()
                }
            };
            let customer_email = {
                let trimmed = req.customer_email.trim();
                if trimmed.is_empty() || !trimmed.contains('@') {
                    "customer@topup-odgj.id".to_string()
                } else {
                    trimmed.to_string()
                }
            };
            let customer_phone = req.customer_phone.and_then(|p| {
                let clean: String = p.chars().filter(|c| c.is_ascii_digit() || *c == '+').collect();
                let digits_only: String = clean.chars().filter(|c| c.is_ascii_digit()).collect();
                if digits_only.is_empty() || digits_only.len() > 20 {
                    None
                } else {
                    Some(clean)
                }
            });

            Customer {
                name: customer_name,
                email: customer_email,
                phone: customer_phone,
                address: req.customer_address.and_then(|a| {
                    let t = a.trim().to_string();
                    if t.is_empty() { None } else { Some(t) }
                }),
                country: Some("ID".to_string()),
            }
        },
        additional_info: None,
    };

    match state.client.checkout().initiate_payment(&checkout_req).await {
        Ok(resp) => {
            if let Some(payment) = resp.response.as_ref().and_then(|r| r.payment.as_ref()) {
                let url = payment.url.clone().unwrap_or_default();
                let token = payment.token.clone();
                (
                    StatusCode::OK,
                    Json(CreateCheckoutResponse {
                        success: true,
                        invoice_number,
                        payment_url: url,
                        token,
                        message: None,
                        raw: Some(resp.raw),
                    }),
                )
            } else {
                (
                    StatusCode::BAD_GATEWAY,
                    Json(CreateCheckoutResponse {
                        success: false,
                        invoice_number,
                        payment_url: String::new(),
                        token: None,
                        message: Some("No payment url returned from DOKU".to_string()),
                        raw: Some(resp.raw),
                    }),
                )
            }
        }
        Err(err) => {
            tracing::error!(error = %err, "doku initiate_payment error");
            (
                StatusCode::BAD_GATEWAY,
                Json(CreateCheckoutResponse {
                    success: false,
                    invoice_number,
                    payment_url: String::new(),
                    token: None,
                    message: Some(err.to_string()),
                    raw: None,
                }),
            )
        }
    }
}

pub async fn order_status(
    State(state): State<Arc<AppState>>,
    Path(invoice_number): Path<String>,
) -> impl IntoResponse {
    if state.config.mock_mode {
        let orders = state.mock_orders.read().await;
        if let Some(order) = orders.get(&invoice_number) {
            return (
                StatusCode::OK,
                Json(OrderStatusResponse {
                    success: true,
                    invoice_number: order.invoice_number.clone(),
                    status: order.status.clone(),
                    amount: Some(order.amount),
                    date: Some(chrono::Utc::now().to_rfc3339()),
                    raw: None,
                }),
            );
        }

        return (
            StatusCode::OK,
            Json(OrderStatusResponse {
                success: true,
                invoice_number: invoice_number.clone(),
                status: "SUCCESS".to_string(),
                amount: None,
                date: Some(chrono::Utc::now().to_rfc3339()),
                raw: None,
            }),
        );
    }

    match state.client.orders().check_status(&invoice_number).await {
        Ok(resp) => {
            let status = resp
                .transaction
                .as_ref()
                .and_then(|t| t.status.clone())
                .unwrap_or_else(|| "PENDING".to_string());
            let amount = resp.order.as_ref().and_then(|o| o.amount);
            let date = resp.transaction.as_ref().and_then(|t| t.date.clone());

            (
                StatusCode::OK,
                Json(OrderStatusResponse {
                    success: true,
                    invoice_number,
                    status,
                    amount,
                    date,
                    raw: Some(resp.raw),
                }),
            )
        }
        Err(err) => {
            tracing::error!(error = %err, invoice = %invoice_number, "doku check_status error");
            (
                StatusCode::NOT_FOUND,
                Json(OrderStatusResponse {
                    success: false,
                    invoice_number,
                    status: "UNKNOWN".to_string(),
                    amount: None,
                    date: None,
                    raw: None,
                }),
            )
        }
    }
}

pub async fn verify_notification(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyNotificationRequest>,
) -> impl IntoResponse {
    let is_valid = if state.config.mock_mode && req.signature == "test_valid_signature" {
        true
    } else {
        state.client.webhook().verify_direct(
            &req.client_id,
            &req.request_id,
            &req.timestamp,
            &req.path,
            &req.body,
            &req.signature,
        )
    };

    if !is_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(VerifyNotificationResponse {
                valid: false,
                invoice_number: None,
                status: None,
                amount: None,
                date: None,
                error: Some("Invalid notification signature".to_string()),
                raw: None,
            }),
        );
    }

    let parsed: Result<DirectWebhookNotification, _> = serde_json::from_str(&req.body);
    match parsed {
        Ok(notif) => {
            let invoice = notif.order.as_ref().map(|o| o.invoice_number.clone());
            let status = notif
                .transaction
                .as_ref()
                .and_then(|t| t.status.clone())
                .unwrap_or_else(|| "SUCCESS".to_string());
            let amount = notif.order.as_ref().and_then(|o| o.amount);
            let date = notif.transaction.as_ref().and_then(|t| t.date.clone());

            if let Some(ref inv) = invoice {
                let mut orders = state.mock_orders.write().await;
                if let Some(ord) = orders.get_mut(inv) {
                    ord.status = status.clone();
                }
            }

            (
                StatusCode::OK,
                Json(VerifyNotificationResponse {
                    valid: true,
                    invoice_number: invoice,
                    status: Some(status),
                    amount,
                    date,
                    error: None,
                    raw: Some(notif.raw),
                }),
            )
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(VerifyNotificationResponse {
                valid: true,
                invoice_number: None,
                status: None,
                amount: None,
                date: None,
                error: Some(format!("Failed to parse notification JSON: {}", err)),
                raw: None,
            }),
        ),
    }
}
