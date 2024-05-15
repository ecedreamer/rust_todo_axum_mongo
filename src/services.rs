use crate::{
    models::{Category, Todo},
    mongoservices,
};
use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;

pub async fn get_todos() -> Result<Vec<Todo>, mongodb::error::Error> {
    let documents: Vec<Todo> = mongoservices::find("todos").await.unwrap();
    // let documents: Vec<Todo> = mongoservices::find::<Todo>("todos").await.unwrap();
    Ok(documents)
}

pub async fn get_todo(_id: ObjectId) -> mongodb::error::Result<Option<Todo>> {
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
    mongoservices::update_one("todos", object_id, todo).await
}

pub async fn delete_todo(
    object_id: ObjectId,
) -> mongodb::error::Result<mongodb::results::DeleteResult> {
    mongoservices::delete_one::<Todo>("todos", object_id).await
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

pub async fn update_category(
    object_id: ObjectId,
    category: Category,
) -> mongodb::error::Result<Option<Category>> {
    mongoservices::update_one("categories", object_id, category).await
}

pub async fn delete_category(
    object_id: ObjectId,
) -> mongodb::error::Result<mongodb::results::DeleteResult> {
    mongoservices::delete_one::<Todo>("categories", object_id).await
}
