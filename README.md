# doku-sdk-rust

[![Crates.io](https://img.shields.io/crates/v/doku-sdk.svg)](https://crates.io/crates/doku-sdk)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

An idiomatic, type-safe, asynchronous Rust SDK for integrating with the **DOKU Payment Gateway**. Fully reverse-engineered and structured from the official DOKU API Collection, supporting both standard **DOKU Direct / Checkout** (Non-SNAP) and Bank Indonesia **SNAP Open API** standards.

---

## Features

- **DOKU Checkout**: Hosted multi-channel payment page session initiation.
- **DOKU Direct (Non-SNAP)**:
  - **Virtual Accounts**: Mandiri, BCA, BNI, BRI, Permata, CIMB, BSI, DOKU (generate and update payment codes).
  - **Credit Cards**: Payment page token generation and online refund.
  - **E-Money**: OVO payment & void, ShopeePay order.
  - **O2O (Convenience Stores)**: Alfamart & Indomaret payment codes with custom receipt messages.
  - **Paylater**: Akulaku order generation, cancellation, and refund.
  - **Direct Transfer**: BCA direct transfer order generation.
  - **Direct Debit**: BRI direct debit token registration, OTP validation, and payments.
- **Bank Indonesia SNAP Open API**:
  - Asymmetric RSA SHA256 (`SHA256withRSA`) signing for B2B Access Tokens (`/authorization/v1/access-token/b2b`).
  - Symmetric HMAC-SHA512 signing for transactional requests.
  - SNAP Virtual Accounts (BNC, BNI, BRI, BSI, BTN, Danamon, Mandiri, Maybank, Permata, Sinarmas, CIMB).
  - SNAP Direct Debit & E-Wallets (account/card binding, OTP, Host-to-Host payment, balance inquiry, refund, unbinding).
- **DOKU Sub Account**: Complete V1 and V2 API support (balance inquiries, transactions, transfers, payouts, debits, splits).
- **Check Status API**: Query order status by invoice number.
- **Webhook Verification**: Pure-Rust verification of incoming notification signatures for both Non-SNAP (`HMAC-SHA256`) and SNAP (`HMAC-SHA512`).

---

## Installation

Add `doku-sdk` to your `Cargo.toml`:

```toml
[dependencies]
doku-sdk = "0.1"
tokio = { version = "1", features = ["full"] }
```

---

## Quickstart

### 1. Initializing the Client

```rust
use doku_sdk::{DokuClient, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DokuClient::builder()
        .client_id("YOUR_CLIENT_ID")
        .secret_key("YOUR_SECRET_KEY")
        // Optional RSA private key in PEM format for SNAP B2B Token API:
        .private_key("-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----")
        .sandbox() // or .production()
        .build()?;

    Ok(())
}
```

---

### 2. DOKU Checkout (Hosted Payment Page)

```rust
use doku_sdk::{CheckoutPayment, CheckoutRequest, Customer, DokuClient, LineItem, Order};

let request = CheckoutRequest {
    order: Order {
        invoice_number: format!("INV-{}", chrono::Utc::now().timestamp()),
        amount: Some(10500),
        currency: Some("IDR".to_string()),
        callback_url: Some("https://myshop.com/payment/callback".to_string()),
        auto_redirect: Some(true),
        line_items: Some(vec![
            LineItem {
                name: "DOKU T-Shirt".to_string(),
                price: 10500,
                quantity: 1,
            }
        ]),
        session_id: None,
    },
    payment: Some(CheckoutPayment {
        payment_due_date: Some(60), // minutes
        payment_method_types: None,
    }),
    customer: Customer {
        name: "Anton Budiman".to_string(),
        email: "anton@example.com".to_string(),
        phone: Some("+6285694566147".to_string()),
        address: Some("Plaza Asia Office Park Unit 3".to_string()),
        country: Some("ID".to_string()),
    },
    additional_info: None,
};

let response = client.checkout().initiate_payment(&request).await?;

if let Some(payment) = response.response.and_then(|r| r.payment) {
    println!("Redirect user to: {:?}", payment.url);
}
```

---

### 3. Non-SNAP Virtual Account (BCA, Mandiri, etc.)

```rust
use doku_sdk::{Customer, GeneratePaymentCodeRequest, Order, VaChannel, VirtualAccountInfo};

let req = GeneratePaymentCodeRequest {
    order: Order {
        invoice_number: "INV-VA-001".to_string(),
        amount: Some(50000),
        ..Default::default()
    },
    virtual_account_info: Some(VirtualAccountInfo {
        reusable_status: Some(true),
        billing_type: Some("NO_BILL".to_string()),
        virtual_account_number: Some("021862040000016".to_string()),
        ..Default::default()
    }),
    customer: Customer {
        name: "Anton Budiman".to_string(),
        email: "anton@example.com".to_string(),
        ..Default::default()
    },
    additional_info: None,
};

let res = client.virtual_account().generate_payment_code(VaChannel::Bca, &req).await?;
if let Some(va) = res.virtual_account_info {
    println!("VA Number: {:?}", va.virtual_account_number);
    println!("How to pay: {:?}", va.how_to_pay_page);
}
```

---

### 4. Bank Indonesia SNAP (Open API Standard)

```rust
use doku_sdk::{SnapAmount, SnapBank, SnapCreateVaRequest};

// 1. Obtain B2B Access Token using asymmetric RSA signature
let token_response = client.snap().get_b2b_token().await?;
let access_token = token_response.access_token;

// 2. Create Virtual Account via SNAP protocol
let va_req = SnapCreateVaRequest {
    partner_service_id: "02186".to_string(),
    customer_no: Some("2040000016".to_string()),
    virtual_account_no: Some("021862040000016".to_string()),
    virtual_account_name: "Customer VA Name".to_string(),
    virtual_account_email: Some("customer@example.com".to_string()),
    virtual_account_phone: Some("6281200001111".to_string()),
    trx_id: "INV-SNAP-001".to_string(),
    total_amount: SnapAmount::idr("12500.00"),
    virtual_account_trx_type: Some("1".to_string()),
    expired_date: Some("2026-11-17T23:59:59+07:00".to_string()),
    additional_info: None,
};

let va_res = client.snap().create_va(SnapBank::Mandiri, &va_req, &access_token).await?;
println!("VA Created: {:?}", va_res.virtual_account_data.and_then(|d| d.virtual_account_no));
```

---

### 5. Webhook Signature Verification

#### Non-SNAP Direct Notification:
```rust
use doku_sdk::webhook::verify_direct_notification;

let is_valid = verify_direct_notification(
    &client_id,
    &request_id,
    &timestamp,
    "/api/payments/doku/callback", // your notification endpoint path
    &raw_request_body,
    &signature_header,             // "HMACSHA256=..."
    "YOUR_SECRET_KEY",
);

if is_valid {
    println!("Notification verified successfully!");
}
```

#### SNAP Webhook Notification:
```rust
use doku_sdk::webhook::verify_snap_notification;

let is_valid = verify_snap_notification(
    "POST",
    "/bi-snap-va/v1/payment",
    &access_token,
    &raw_request_body,
    &x_timestamp_header,
    &x_signature_header,
    "YOUR_SECRET_KEY",
);
```

---

### 6. Checking Transaction Status

```rust
let status = client.orders().check_status("INV-1640746942").await?;
println!("Status: {:?}", status.transaction.and_then(|t| t.status));
```

---

## Running Examples and Tests

Run the full automated test suite:
```bash
cargo test
```

Check code quality with Clippy:
```bash
cargo clippy --all-targets -- -D warnings
```

Run the webhook verification example:
```bash
cargo run --example webhook_verification_example
```

---

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
