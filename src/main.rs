use axum::{routing::get, Router};
use rust_todo_axum_mongo::routers::{category_router, todo_router};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use axum::{http::StatusCode, response::IntoResponse, Json};

use serde_json::json;

pub async fn home() -> impl IntoResponse {
    tracing::info!("Page: home");
    (StatusCode::OK, Json(json!({"status": "ok"})))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let origins = ["127.0.0.1".parse().unwrap(), "localhost".parse().unwrap()];
    let cors = CorsLayer::new().allow_origin(origins);

    let app = Router::new()
        .route("/", get(home))
        .merge(category_router::get_category_router().await)
        .merge(todo_router::get_todo_router().await)
        .layer(ServiceBuilder::new().layer(cors));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
