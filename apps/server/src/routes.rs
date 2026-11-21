use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::{handlers, AppState};

pub fn router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(handlers::health))
        .route("/api/v1/health", get(handlers::health))
        .route("/api/v1/checkout", post(handlers::checkout))
        .route("/api/v1/orders/{invoice}/status", get(handlers::order_status))
        .route("/api/v1/notifications/verify", post(handlers::verify_notification))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
