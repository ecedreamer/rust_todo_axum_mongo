use crate::{models::{Category, Todo}, mongoservices};
use chrono::{DateTime, Local};
use mongodb::{bson, bson::oid::ObjectId};

use tracing::info;


pub async fn get_todos() -> Result<Vec<Todo>, mongodb::error::Error> {
    let documents: Vec<Todo> = mongoservices::find("todos").await.unwrap();
    // let documents: Vec<Todo> = mongoservices::find::<Todo>("todos").await.unwrap();
    Ok(documents)
}

pub async fn get_todo(_id: ObjectId) -> mongodb::error::Result<Option<Todo>> {
    // let db = mongoservices::establish_mongodb_connection().await?;
    // let collection = db.collection::<Todo>("todos");

    // let filter = bson::doc! {"_id": _id};
    // collection.find_one(filter, None).await
    mongoservices::find_one("todos", _id).await
}

pub async fn add_todo(mut todo: Todo) -> mongodb::error::Result<Option<Todo>> {

    if todo.date.is_none() {
        let current_datetime: DateTime<Local> = Local::now();

        let formatted_datetime = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        todo.date = Some(formatted_datetime);
    }
    if todo.status.is_none() {
        todo.date = Some("Open".to_string());
    }

    mongoservices::insert_one("todos", todo).await
}

pub async fn update_todo(object_id: ObjectId, todo: Todo) -> mongodb::error::Result<Option<Todo>> {
    let db = mongoservices::establish_mongodb_connection().await?;
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
    let db = mongoservices::establish_mongodb_connection().await?;
    let collection = db.collection::<Todo>("todos");

    let filter = bson::doc! {"_id": object_id};
    let result = collection.delete_one(filter, None).await;
    // info!("Deleted documents: {:?}", result);
    result
}

pub async fn get_categories() -> Result<Vec<Category>, mongodb::error::Error> {
    let documents: Vec<Category> = mongoservices::find("categories").await.unwrap();
    Ok(documents)
}

pub async fn get_category(_id: ObjectId) -> mongodb::error::Result<Option<Category>> {
    mongoservices::find_one("categories", _id).await
}

pub async fn add_category(category: Category) -> mongodb::error::Result<Option<Category>> {
    mongoservices::insert_one("categories", category).await
}