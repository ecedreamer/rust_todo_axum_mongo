use axum::{
    routing::{delete, get, post, put},
    Router,
};
use rust_todo_axum_mongo::handlers;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let origins = ["127.0.0.1".parse().unwrap(), "localhost".parse().unwrap()];
    let cors = CorsLayer::new().allow_origin(origins);

    let app = Router::new()
        .route("/", get(handlers::home))
        .route("/todos", get(handlers::list_todos))
        .route("/todos/:todo_id", get(handlers::get_todo))
        .route("/todos", post(handlers::add_todo))
        .route("/todos/:todo_id", put(handlers::update_todo))
        .route("/todos/:todo_id", delete(handlers::delete_todo))
        .route("/categories", get(handlers::list_categories))
        .route("/categories/:category_id", get(handlers::get_category))
        .route("/categories", post(handlers::add_category))
        .route("/categories/:category_id", put(handlers::update_category))
        .route(
            "/categories/:category_id",
            delete(handlers::delete_category),
        )
        .layer(ServiceBuilder::new().layer(cors));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
