use crate::models::Todo;
use chrono::{DateTime, Local};
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, options::ClientOptions, Client, Database};
use std::env;
use urlencoding::encode;

use tracing::info;

pub async fn establish_mongodb_connection() -> Result<Database, mongodb::error::Error> {
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not set");
    let mongo_username = env::var("MONGO_USERNAME").expect("MONGO_USERNAME not set");
    let raw_password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD not set");
    let mongo_password = encode(&raw_password);
    // let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let connection_string =
        format!("mongodb://{mongo_username}:{mongo_password}@{mongo_url}/?authSource=admin");
    let client_options = ClientOptions::parse(connection_string).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("rs_test_db");
    Ok(db)
}

pub async fn get_todos() -> Result<Vec<Todo>, mongodb::error::Error> {
    let db = establish_mongodb_connection().await?;
    let collection = db.collection::<Todo>("todos");

    let filter = bson::doc! {};
    let mut cursor = collection.find(filter, None).await.unwrap();

    let mut documents = Vec::new();

    while let Some(post) = cursor.next().await {
        documents.push(post?);
    }
    Ok(documents)
}

pub async fn get_todo(_id: ObjectId) -> mongodb::error::Result<Option<Todo>> {
    let db = establish_mongodb_connection().await?;
    let collection = db.collection::<Todo>("todos");

    let filter = bson::doc! {"_id": _id};
    collection.find_one(filter, None).await
}

pub async fn add_todo(mut todo: Todo) -> mongodb::error::Result<Option<Todo>> {
    let db = establish_mongodb_connection().await?;
    let collection = db.collection::<Todo>("todos");

    if todo.date.is_none() {
        let current_datetime: DateTime<Local> = Local::now();

        let formatted_datetime = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        todo.date = Some(formatted_datetime);
    }
    if todo.status.is_none() {
        todo.date = Some("Open".to_string());
    }

    let insert_result = collection.insert_one(todo, None).await;
    get_todo(insert_result.unwrap().inserted_id.as_object_id().unwrap()).await
}

pub async fn update_todo(object_id: ObjectId, todo: Todo) -> mongodb::error::Result<Option<Todo>> {
    let db = establish_mongodb_connection().await?;
    let collection = db.collection::<Todo>("todos");

    let query = bson::doc! {"_id": object_id};

    let todo_doc = bson::to_document(&todo)?;

    let _update_result = collection
        .update_one(query, bson::doc! {"$set": todo_doc}, None)
        .await?;

    if _update_result.modified_count > 0 {
        get_todo(object_id).await
    } else {
        info!("Todo not updated");
        get_todo(object_id).await
    }
}

pub async fn delete_todo(
    object_id: ObjectId,
) -> mongodb::error::Result<mongodb::results::DeleteResult> {
    let db = establish_mongodb_connection().await?;
    let collection = db.collection::<Todo>("todos");

    let filter = bson::doc! {"_id": object_id};
    let result = collection.delete_one(filter, None).await;
    // info!("Deleted documents: {:?}", result);
    result
}
