use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub enum Status {
    Open,
    Doing,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub content: String,
    pub date: Option<String>,
    pub status: Option<Status>,
}
