use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use crate::{models::Todo, services};
use mongodb::bson;
use serde_json::json;
use std::str::FromStr;

pub async fn list_todos() -> impl IntoResponse {
    tracing::info!("Page: list todos");
    let todos = services::get_todos().await.unwrap();
    (StatusCode::OK, Json(todos))
}

pub async fn get_todo(Path(todo_id): Path<String>) -> impl IntoResponse {
    tracing::info!("Page: get a todo with {}", &todo_id);

    if let Ok(object_id) = bson::oid::ObjectId::from_str(&todo_id) {
        match services::get_todo(object_id).await {
            Ok(Some(todo)) => (StatusCode::OK, Json(json!(todo))),
            Ok(None) => (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Todo not found"})),
            ),
            Err(err) => (
                StatusCode::NOT_FOUND,
                Json(json!({"message": err.to_string()})),
            ),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Invalid ObjectId"})),
        )
    }
}

pub async fn add_todo(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    tracing::info!("Page: get a todo");
    if let Ok(validated_data) = serde_json::from_value::<Todo>(payload) {
        let add_result = services::add_todo(validated_data).await;
        match add_result {
            Ok(Some(todo)) => (StatusCode::CREATED, Json(json!(todo))),
            Ok(None) => (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Todo not found"})),
            ),
            Err(err) => (
                StatusCode::NOT_FOUND,
                Json(json!({"message": err.to_string()})),
            ),
        }
    } else {
        tracing::error!("Failed to deserialize JSON into YourStruct");
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Invalid payload"})),
        )
    }
}

pub async fn update_todo(
    Path(todo_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    tracing::info!("Page: update a todo");
    if let Ok(object_id) = bson::oid::ObjectId::from_str(&todo_id) {
        if let Ok(validated_data) = serde_json::from_value::<Todo>(payload) {
            let update_result = services::update_todo(object_id, validated_data).await;
            match update_result {
                Ok(Some(todo)) => (StatusCode::OK, Json(json!(todo))),
                Ok(None) => (
                    StatusCode::NOT_FOUND,
                    Json(json!({"message": "Todo not found"})),
                ),
                Err(err) => (
                    StatusCode::NOT_FOUND,
                    Json(json!({"message": err.to_string()})),
                ),
            }
        } else {
            tracing::error!("Failed to deserialize JSON into YourStruct");
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Invalid payload"})),
            )
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Invalid ObjectId"})),
        )
    }
}

pub async fn delete_todo(Path(todo_id): Path<String>) -> impl IntoResponse {
    tracing::info!("Page: delete a todo");
    if let Ok(object_id) = bson::oid::ObjectId::from_str(&todo_id) {
        let _delete_result = services::delete_todo(object_id).await;
        match _delete_result {
            Ok(result) => {
                if result.deleted_count == 1 {
                    (
                        StatusCode::OK,
                        Json(
                            json!({"message": format!("Todo with id {todo_id} deleted successfully")}),
                        ),
                    )
                } else {
                    (
                        StatusCode::OK,
                        Json(json!({"message": format!("Todo with id {todo_id} not found")})),
                    )
                }
            }
            Err(err) => (
                StatusCode::OK,
                Json(json!({"message": format!("Error in deleting todo. {err}")})),
            ),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Invalid ObjectId"})),
        )
    }
}
