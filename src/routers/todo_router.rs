use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::route_handlers::todo_handlers;

pub async fn get_todo_router() -> Router {
    Router::new()
        .route("/todos", get(todo_handlers::list_todos))
        .route("/todos/:todo_id", get(todo_handlers::get_todo))
        .route("/todos", post(todo_handlers::add_todo))
        .route("/todos/:todo_id", put(todo_handlers::update_todo))
        .route("/todos/:todo_id", delete(todo_handlers::delete_todo))
}
