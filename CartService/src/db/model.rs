use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cart {
    pub id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub items: Option<List<CartItem>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CartItem {
    pub id: Uuid,
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    #[serde(rename = "addedAt")]
    pub added_at: DateTime<Utc>,
}