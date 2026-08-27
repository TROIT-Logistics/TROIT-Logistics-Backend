use crate::{
    auth::handlers::{login_handler, logout_handler, me_handler, register_handler},
    middleware::require_auth,
    models::AppState,
};
use axum::{
    middleware,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

/// Infrastructure Health Check Handler: GET /health
pub async fn health_handler() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "TROIT Logistics Backend API",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Placeholder handler for future domain API endpoints
pub async fn placeholder_domain_handler() -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Domain route foundation active. Feature endpoints to be implemented by assigned developers."
    }))
}

/// Constructs complete Axum application router hierarchy
pub fn create_router(state: AppState) -> Router {
    // Auth routes under /api/v1/auth
    let public_auth_routes = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler));

    let protected_auth_routes = Router::new()
        .route("/me", get(me_handler))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let auth_routes = Router::new()
        .merge(public_auth_routes)
        .merge(protected_auth_routes);

    // Placeholder domain routers establishing API versioning convention /api/v1/
    let domain_routes = Router::new()
        .route("/users", get(placeholder_domain_handler))
        .route("/products", get(placeholder_domain_handler))
        .route("/orders", get(placeholder_domain_handler))
        .route("/logistics", get(placeholder_domain_handler));

    // Combine all v1 API routes
    let api_v1_routes = Router::new()
        .nest("/auth", auth_routes)
        .merge(domain_routes);

    // Root Router with /health and /api/v1/
    Router::new()
        .route("/health", get(health_handler))
        .nest("/api/v1", api_v1_routes)
        .with_state(state)
}
