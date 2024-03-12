use axum::{
    routing::{delete, get, post, put},
    Router,
};

use rust_todo_axum_mongo::handlers::{
    add_todo, delete_todo, get_todo, home, list_todos, update_todo,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(home))
        .route("/todos", get(list_todos))
        .route("/todos/:todo_id", get(get_todo))
        .route("/todos", post(add_todo))
        .route("/todos/:todo_id", put(update_todo))
        .route("/todos/:todo_id", delete(delete_todo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
