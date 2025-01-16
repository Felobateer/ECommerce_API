use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::services::{CartService, CartServiceError, CartItem, AddCartItemRequest};
use uuid::Uuid;

pub async fn get_cart(
    Path(user_id): Path<Uuid>,
    State(cart_service): State<CartService>,
) -> impl IntoResponse {
    match cart_service.get_cart(user_id).await {
        Ok(cart) => Json(cart).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch cart").into_response(),
    }
}

pub async fn add_cart_item(
    Path(user_id): Path<Uuid>,
    State(cart_service): State<CartService>,
    Json(payload): Json<AddCartItemRequest>,
) -> impl IntoResponse {
    match cart_service
        .add_item_to_cart(user_id, payload.product_id, payload.quantity)
        .await
    {
        Ok(_) => (StatusCode::CREATED, "Item added to cart").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add item").into_response(),
    }
}

pub async fn remove_cart_item(
    Path((user_id, item_id)): Path<(Uuid, Uuid)>,
    State(cart_service): State<CartService>,
) -> impl IntoResponse {
    match cart_service.remove_item_from_cart(user_id, item_id).await {
        Ok(_) => (StatusCode::OK, "Item removed from cart").into_response(),
        Err(CartServiceError::ItemNotFound) => (StatusCode::NOT_FOUND, "Item not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to remove item").into_response(),
    }
}

pub async fn clear_cart(
    Path(user_id): Path<Uuid>,
    State(cart_service): State<CartService>,
) -> impl IntoResponse {
    match cart_service.clear_cart(user_id).await {
        Ok(_) => (StatusCode::OK, "Cart cleared").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to clear cart").into_response(),
    }
}
