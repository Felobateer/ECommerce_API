use sqlx::MySqlPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CartItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cart {
    pub user_id: Uuid,
    pub items: Vec<CartItem>,
}

#[derive(Debug, Error)]
pub enum CartServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Cart item not found")]
    ItemNotFound,
}

pub struct CartService {
    pub db_pool: MySqlPool,
}

impl CartService {
    // Retrieve a user's cart
    pub async fn get_cart(&self, user_id: Uuid) -> Result<Cart, CartServiceError> {
        let items = sqlx::query_as!(
            CartItem,
            r#"
            SELECT id, product_id, quantity
            FROM cart_items
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(Cart { user_id, items })
    }

    // Add an item to the cart
    pub async fn add_item_to_cart(
        &self,
        user_id: Uuid,
        product_id: Uuid,
        quantity: u32,
    ) -> Result<(), CartServiceError> {
        let item_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO cart_items (id, user_id, product_id, quantity)
            VALUES (?, ?, ?, ?)
            "#,
            item_id,
            user_id,
            product_id,
            quantity
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    // Remove an item from the cart
    pub async fn remove_item_from_cart(
        &self,
        user_id: Uuid,
        item_id: Uuid,
    ) -> Result<(), CartServiceError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM cart_items
            WHERE user_id = ? AND id = ?
            "#,
            user_id,
            item_id
        )
        .execute(&self.db_pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(CartServiceError::ItemNotFound);
        }

        Ok(())
    }

    // Clear the user's cart
    pub async fn clear_cart(&self, user_id: Uuid) -> Result<(), CartServiceError> {
        sqlx::query!(
            r#"
            DELETE FROM cart_items
            WHERE user_id = ?
            "#,
            user_id
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
