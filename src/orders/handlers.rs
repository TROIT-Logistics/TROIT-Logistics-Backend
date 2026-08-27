use crate::{
    errors::AppError,
    models::{AppState, Claims, UserRole},
    orders::models::{
        CreateOrderRequest, CreatePickupInspectionRequest, Order, OrderResponse, PickupInspection,
        UpdateOrderStatusRequest,
    },
    products::models::Product,
};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::Serialize;
use sqlx::query_as;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

/// POST /api/v1/orders
/// Buyers place an order for a verified product
pub async fn create_order_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<ApiResponse<OrderResponse>>, AppError> {
    let quantity = payload.quantity.unwrap_or(1);
    if quantity <= 0 {
        return Err(AppError::ValidationError(
            "Order quantity must be greater than zero".to_string(),
        ));
    }

    // Fetch product
    let product: Product = query_as::<_, Product>(
        "SELECT id, seller_id, name, description, price, condition, stock, verification_status, created_at, updated_at FROM products WHERE id = $1"
    )
    .bind(payload.product_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

    // Check product is verified for buyer purchase
    if product.verification_status != "VERIFIED" {
        return Err(AppError::BadRequest(
            "Only verified products can be ordered".to_string(),
        ));
    }

    // Check seller is not buying their own product
    if product.seller_id == claims.sub {
        return Err(AppError::BadRequest(
            "Sellers cannot place orders on their own products".to_string(),
        ));
    }

    // Check stock availability
    if product.stock < quantity {
        return Err(AppError::Conflict(format!(
            "Insufficient stock. Available: {}, Requested: {}",
            product.stock, quantity
        )));
    }

    // Calculate total amount
    let total_amount = product.price * (quantity as f64);

    // Deduct stock
    sqlx::query("UPDATE products SET stock = stock - $1 WHERE id = $2")
        .bind(quantity)
        .bind(product.id)
        .execute(&state.db)
        .await?;

    // Insert order with initial status CONFIRMED and payment_status PROTECTED
    let order: Order = query_as::<_, Order>(
        r#"
        INSERT INTO orders (buyer_id, seller_id, product_id, quantity, amount, status, payment_status, delivery_status)
        VALUES ($1, $2, $3, $4, $5, 'CONFIRMED', 'PROTECTED', 'PENDING')
        RETURNING id, buyer_id, seller_id, product_id, quantity, amount, status, payment_status, delivery_status, created_at, updated_at
        "#
    )
    .bind(claims.sub)
    .bind(product.seller_id)
    .bind(product.id)
    .bind(quantity)
    .bind(total_amount)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        message: "Order placed successfully. Payment is in PROTECTED escrow state.".to_string(),
        data: Some(order.to_response()),
    }))
}

/// GET /api/v1/orders
/// List orders for the authenticated user (either as buyer or seller)
pub async fn list_orders_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<OrderResponse>>>, AppError> {
    let orders: Vec<Order> = query_as::<_, Order>(
        r#"
        SELECT id, buyer_id, seller_id, product_id, quantity, amount, status, payment_status, delivery_status, created_at, updated_at
        FROM orders
        WHERE buyer_id = $1 OR seller_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(claims.sub)
    .fetch_all(&state.db)
    .await?;

    let response_data = orders.into_iter().map(|o| o.to_response()).collect();

    Ok(Json(ApiResponse {
        success: true,
        message: "Orders retrieved successfully".to_string(),
        data: Some(response_data),
    }))
}

/// GET /api/v1/orders/:id
/// Gets details for a specific order with ownership verification
pub async fn get_order_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<OrderResponse>>, AppError> {
    let order: Order = query_as::<_, Order>(
        r#"
        SELECT id, buyer_id, seller_id, product_id, quantity, amount, status, payment_status, delivery_status, created_at, updated_at
        FROM orders
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

    // Authorization check
    if order.buyer_id != claims.sub
        && order.seller_id != claims.sub
        && claims.role != UserRole::Admin
    {
        return Err(AppError::Forbidden(
            "You are not authorized to view this order".to_string(),
        ));
    }

    Ok(Json(ApiResponse {
        success: true,
        message: "Order details retrieved successfully".to_string(),
        data: Some(order.to_response()),
    }))
}

/// PATCH /api/v1/orders/:id/status
/// Update order status through lifecycle (CONFIRMED, READY_FOR_PICKUP, OUT_FOR_DELIVERY, DELIVERED)
pub async fn update_order_status_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<Json<ApiResponse<OrderResponse>>, AppError> {
    let new_status = payload.status.to_uppercase();
    let valid_statuses = [
        "CONFIRMED",
        "READY_FOR_PICKUP",
        "OUT_FOR_DELIVERY",
        "DELIVERED",
        "COMPLETED",
        "CANCELLED",
    ];

    if !valid_statuses.contains(&new_status.as_str()) {
        return Err(AppError::ValidationError(format!(
            "Invalid status '{}'. Must be one of: {:?}",
            new_status, valid_statuses
        )));
    }

    let existing_order: Order = query_as::<_, Order>(
        "SELECT id, buyer_id, seller_id, product_id, quantity, amount, status, payment_status, delivery_status, created_at, updated_at FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

    // Authorization: seller, rider, or admin can update delivery lifecycle statuses
    if existing_order.seller_id != claims.sub
        && claims.role != UserRole::Rider
        && claims.role != UserRole::Admin
        && claims.role != UserRole::FieldAgent
    {
        return Err(AppError::Forbidden(
            "Not authorized to update order status".to_string(),
        ));
    }

    let delivery_status_update = match new_status.as_str() {
        "READY_FOR_PICKUP" => "PICKUP_READY",
        "OUT_FOR_DELIVERY" => "IN_TRANSIT",
        "DELIVERED" => "DELIVERED",
        "COMPLETED" => "DELIVERED",
        _ => &existing_order.delivery_status,
    };

    let updated_order: Order = query_as::<_, Order>(
        r#"
        UPDATE orders
        SET status = $1, delivery_status = $2, updated_at = CURRENT_TIMESTAMP
        WHERE id = $3
        RETURNING id, buyer_id, seller_id, product_id, quantity, amount, status, payment_status, delivery_status, created_at, updated_at
        "#
    )
    .bind(&new_status)
    .bind(delivery_status_update)
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        message: format!("Order status updated to {}", new_status),
        data: Some(updated_order.to_response()),
    }))
}

/// POST /api/v1/orders/:id/pickup-inspection
/// Records rider/agent pickup inspection for the order
pub async fn create_pickup_inspection_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreatePickupInspectionRequest>,
) -> Result<Json<ApiResponse<PickupInspection>>, AppError> {
    let order: Order = query_as::<_, Order>(
        "SELECT id, buyer_id, seller_id, product_id, quantity, amount, status, payment_status, delivery_status, created_at, updated_at FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

    let inspection_status = payload
        .inspection_status
        .unwrap_or_else(|| "PASSED".to_string())
        .to_uppercase();
    if inspection_status != "PASSED"
        && inspection_status != "FAILED"
        && inspection_status != "PENDING"
    {
        return Err(AppError::ValidationError(
            "Inspection status must be PASSED, FAILED, or PENDING".to_string(),
        ));
    }

    let inspection: PickupInspection = query_as::<_, PickupInspection>(
        r#"
        INSERT INTO pickup_inspections (order_id, inspector_id, condition, notes, inspection_status)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, order_id, inspector_id, condition, notes, inspection_status, created_at
        "#,
    )
    .bind(order.id)
    .bind(claims.sub)
    .bind(payload.condition.trim())
    .bind(payload.notes.as_deref().map(|n| n.trim()))
    .bind(&inspection_status)
    .fetch_one(&state.db)
    .await?;

    // If inspection passed, update order status to READY_FOR_PICKUP
    if inspection_status == "PASSED" {
        sqlx::query("UPDATE orders SET status = 'READY_FOR_PICKUP', delivery_status = 'PICKUP_READY', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(order.id)
            .execute(&state.db)
            .await?;
    }

    Ok(Json(ApiResponse {
        success: true,
        message: format!(
            "Pickup inspection recorded with status {}",
            inspection_status
        ),
        data: Some(inspection),
    }))
}

/// POST /api/v1/orders/:id/confirm-delivery
/// Buyer confirms delivery -> Order becomes COMPLETED & Payment status RELEASED
pub async fn confirm_delivery_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<OrderResponse>>, AppError> {
    let order: Order = query_as::<_, Order>(
        "SELECT id, buyer_id, seller_id, product_id, quantity, amount, status, payment_status, delivery_status, created_at, updated_at FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

    // Must be the buyer who owns the order
    if order.buyer_id != claims.sub {
        return Err(AppError::Forbidden(
            "Only the buyer who placed this order can confirm delivery".to_string(),
        ));
    }

    // Must be in DELIVERED status
    if order.status != "DELIVERED" {
        return Err(AppError::BadRequest(format!(
            "Cannot confirm delivery for order in '{}' status. Order must be DELIVERED.",
            order.status
        )));
    }

    // Transition order to COMPLETED and payment to RELEASED
    let updated_order: Order = query_as::<_, Order>(
        r#"
        UPDATE orders
        SET status = 'COMPLETED', payment_status = 'RELEASED', updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, buyer_id, seller_id, product_id, quantity, amount, status, payment_status, delivery_status, created_at, updated_at
        "#
    )
    .bind(order.id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        message: "Delivery confirmed. Order COMPLETED and protected payment RELEASED to seller."
            .to_string(),
        data: Some(updated_order.to_response()),
    }))
}
