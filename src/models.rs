use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub enum Status {
    Open,
    Doing,
    Done,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub category_id: Option<String>,
    pub content: String,
    pub date: Option<String>,
    pub status: Option<Status>,
}

#[derive(Serialize, Debug, Deserialize)]
pub enum ModelStruct {
    ModelStruct(Todo),
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityLog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    related_struct: ModelStruct,
    related_struct_id: Option<ObjectId>,
    activity: String,
    pub date: Option<String>,
}
