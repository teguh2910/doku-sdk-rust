//! # DOKU SDK for Rust
//!
//! An idiomatic, asynchronous Rust SDK for integrating with the **DOKU Payment Gateway**.
//! Supports both **DOKU Direct / Checkout** (Non-SNAP) and Bank Indonesia **SNAP Open API** standards.
//!
//! ## Features
//! - **DOKU Checkout**: Hosted payment pages with multi-channel support.
//! - **DOKU Direct (Non-SNAP)**:
//!   - Virtual Accounts (BCA, Mandiri, BNI, BRI, Permata, CIMB, BSI, DOKU).
//!   - Credit Cards (generate payment page, online refund).
//!   - E-Money (OVO payment & void, ShopeePay).
//!   - O2O (Alfamart, Indomaret).
//!   - Paylater (Akulaku).
//!   - Direct Transfer (BCA).
//!   - Direct Debit (BRI).
//! - **DOKU SNAP (Bank Indonesia Open API standard)**:
//!   - Asymmetric RSA SHA256 (`SHA256withRSA`) signing for B2B tokens.
//!   - Symmetric HMAC-SHA512 signing for transactions.
//!   - SNAP Virtual Accounts (BNC, BNI, BRI, BSI, BTN, Danamon, Mandiri, Maybank, Permata, Sinarmas, CIMB).
//!   - SNAP Direct Debit & E-Wallet (Account & card binding, OTP, Host-to-Host payment, balance inquiry, refund).
//! - **DOKU Sub Account**: V1 & V2 APIs (balance inquiry, transactions, transfers, debits, splits).
//! - **Check Status API**: Query transaction status by invoice number.
//! - **Webhook Notification Verification**: Secure validation of incoming HTTP notifications for both Direct and SNAP standards.

pub mod client;
pub mod config;
pub mod crypto;
pub mod error;
pub mod models;
pub mod services;
pub mod webhook;

pub use client::{DokuClient, DokuClientBuilder};
pub use config::{DokuConfig, DokuConfigBuilder, Environment};
pub use error::{DokuError, Result};
pub use models::*;
pub use services::*;
pub use webhook::{verify_direct_notification, verify_snap_notification, WebhookVerifier};
