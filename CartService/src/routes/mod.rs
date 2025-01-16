use axum::{
    middleware,
    routing::{get, post, delete},
    Router,
};
use crate::{
    handlers::{add_cart_item, clear_cart, get_cart, remove_cart_item},
    middleware::{logger_middleware, auth_middleware},
    services::CartService,
};
use std::sync::Arc;

/// Create the main router for the cart service
pub fn create_router(cart_service: Arc<CartService>) -> Router {
    Router::new()
        .route("/cart/:user_id", get(get_cart)) // Get all items in the user's cart
        .route("/cart/:user_id/add", post(add_cart_item)) // Add an item to the cart
        .route("/cart/:user_id/remove/:item_id", delete(remove_cart_item)) // Remove an item
        .route("/cart/:user_id/clear", delete(clear_cart)) // Clear the user's cart
        .layer(middleware::from_fn(logger_middleware)) // Attach the logger middleware
        .layer(middleware::from_fn_with_state(
            cart_service.clone(),
            auth_middleware,
        )) // Attach authentication middleware with state
        .with_state(cart_service) // Inject the shared CartService
}
