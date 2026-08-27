use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Core Order entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub amount: f64,
    pub status: String,
    pub payment_status: String,
    pub delivery_status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Core PickupInspection entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PickupInspection {
    pub id: Uuid,
    pub order_id: Uuid,
    pub inspector_id: Option<Uuid>,
    pub condition: String,
    pub notes: Option<String>,
    pub inspection_status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub product_id: Uuid,
    pub quantity: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePickupInspectionRequest {
    pub condition: String,
    pub notes: Option<String>,
    pub inspection_status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub amount: f64,
    pub status: String,
    pub payment_status: String,
    pub delivery_status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn to_response(&self) -> OrderResponse {
        OrderResponse {
            id: self.id,
            buyer_id: self.buyer_id,
            seller_id: self.seller_id,
            product_id: self.product_id,
            quantity: self.quantity,
            amount: self.amount,
            status: self.status.clone(),
            payment_status: self.payment_status.clone(),
            delivery_status: self.delivery_status.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
