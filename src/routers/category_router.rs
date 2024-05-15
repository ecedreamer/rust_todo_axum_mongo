use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::route_handlers::category_handlers;

pub async fn get_category_router() -> Router {
    Router::new()
        .route("/categories", get(category_handlers::list_categories))
        .route(
            "/categories/:category_id",
            get(category_handlers::get_category),
        )
        .route("/categories", post(category_handlers::add_category))
        .route(
            "/categories/:category_id",
            put(category_handlers::update_category),
        )
        .route(
            "/categories/:category_id",
            delete(category_handlers::delete_category),
        )
}
