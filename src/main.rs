use axum::{
    routing::{delete, get, post, put},
    Router,
};
use rust_todo_axum_mongo::handlers::{
    add_todo, delete_todo, get_todo, home, list_categories, list_todos, update_todo, get_category, add_category
};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let origins = [
        "127.0.0.1".parse().unwrap(),
        "localhost".parse().unwrap(),
    ];
    let cors = CorsLayer::new()
                            .allow_origin(origins);

    let app = Router::new()
        .route("/", get(home))
        .route("/todos", get(list_todos))
        .route("/todos/:todo_id", get(get_todo))
        .route("/todos", post(add_todo))
        .route("/todos/:todo_id", put(update_todo))
        .route("/todos/:todo_id", delete(delete_todo))
        .route("/categories", get(list_categories))
        .route("/categories/:category_id", get(get_category))
        .route("/categories", post(add_category))
        .layer(
            ServiceBuilder::new().layer(cors)
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
