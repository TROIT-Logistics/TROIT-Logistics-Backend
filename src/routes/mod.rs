use crate::{
    auth::handlers::{login_handler, logout_handler, me_handler, register_handler},
    middleware::require_auth,
    models::AppState,
    orders::handlers::{
        confirm_delivery_handler, create_order_handler, create_pickup_inspection_handler,
        get_order_handler, list_orders_handler, update_order_status_handler,
    },
    products::handlers::{
        create_product_handler, get_product_handler, list_products_handler, verify_product_handler,
    },
    seed::handlers::seed_demo_data_handler,
};
use axum::{
    middleware,
    routing::{get, patch, post},
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

/// Constructs complete Axum application router hierarchy
pub fn create_router(state: AppState) -> Router {
    // 1. Auth routes
    let public_auth = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler));

    let protected_auth = Router::new()
        .route("/me", get(me_handler))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let auth_routes = Router::new().merge(public_auth).merge(protected_auth);

    // 2. Product routes
    let public_products = Router::new()
        .route("/", get(list_products_handler))
        .route("/:id", get(get_product_handler))
        .route("/:id/verify", patch(verify_product_handler));

    let protected_products = Router::new()
        .route("/", post(create_product_handler))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let product_routes = Router::new()
        .merge(public_products)
        .merge(protected_products);

    // 3. Order & Delivery routes (Protected)
    let order_routes = Router::new()
        .route("/", post(create_order_handler))
        .route("/", get(list_orders_handler))
        .route("/:id", get(get_order_handler))
        .route("/:id/status", patch(update_order_status_handler))
        .route(
            "/:id/pickup-inspection",
            post(create_pickup_inspection_handler),
        )
        .route("/:id/confirm-delivery", post(confirm_delivery_handler))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // Combine API v1 routes
    let api_v1 = Router::new()
        .nest("/auth", auth_routes)
        .nest("/products", product_routes)
        .nest("/orders", order_routes)
        .route("/seed", post(seed_demo_data_handler));

    // Root Router
    Router::new()
        .route("/health", get(health_handler))
        .nest("/api/v1", api_v1)
        .with_state(state)
}
