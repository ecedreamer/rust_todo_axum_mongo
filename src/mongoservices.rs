use futures_util::StreamExt;
use mongodb::error::Error as MongoError;
use mongodb::{
    bson::{self, oid::ObjectId, Document},
    options::ClientOptions,
    Client, Database,
};
use std::env;
use urlencoding::encode;

pub async fn establish_mongodb_connection() -> Result<Database, mongodb::error::Error> {
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not set");
    let mongo_username = env::var("MONGO_USERNAME").expect("MONGO_USERNAME not set");
    let raw_password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD not set");
    let mongo_password = encode(&raw_password);

    let connection_string =
        format!("mongodb://{mongo_username}:{mongo_password}@{mongo_url}/?authSource=admin");
    let mut client_options = ClientOptions::parse(connection_string).await?;
    client_options.max_pool_size = Some(10);

    let client = Client::with_options(client_options)?;

    let db = client.database("rs_test_db");
    Ok(db)
}

pub async fn find<T>(collection_name: &str) -> Result<Vec<T>, MongoError>
where
    T: serde::de::DeserializeOwned,
{
    let db = establish_mongodb_connection().await?;
    let collection = db.collection::<T>(collection_name);
    let mut cursor = collection.find(None, None).await?;
    let mut documents = Vec::new();

    while let Some(document) = cursor.next().await {
        documents.push(document?);
    }

    Ok(documents)
}

pub async fn find_one<T>(collection_name: &str, _id: ObjectId) -> mongodb::error::Result<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    let db = establish_mongodb_connection().await?;
    let collection = db.collection::<Document>(collection_name);

    let filter = bson::doc! {"_id": _id};
    if let Some(document) = collection.find_one(filter, None).await? {
        let document = bson::from_document(document)?;
        Ok(Some(document))
    } else {
        Ok(None)
    }
}

pub async fn insert_one<T>(collection_name: &str, data: T) -> mongodb::error::Result<Option<T>>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let db = establish_mongodb_connection().await?;
    let collection = db.collection::<T>(collection_name);

    let insert_result = collection.insert_one(data, None).await;
    find_one(
        collection_name,
        insert_result.unwrap().inserted_id.as_object_id().unwrap(),
    )
    .await
}

pub async fn update_one<T>(
    collection_name: &str,
    _id: ObjectId,
    data: T,
) -> mongodb::error::Result<Option<T>>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let db = establish_mongodb_connection().await?;
    let collection = db.collection::<T>(collection_name);

    let query = bson::doc! {"_id": _id};

    let data_doc = bson::to_document(&data)?;

    let _update_result = collection
        .update_one(query, bson::doc! {"$set": data_doc}, None)
        .await?;

    if _update_result.modified_count > 0 {
        find_one(collection_name, _id).await
    } else {
        find_one(collection_name, _id).await
    }
}

pub async fn delete_one<T>(
    collection_name: &str,
    _id: ObjectId,
) -> mongodb::error::Result<mongodb::results::DeleteResult>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let db = establish_mongodb_connection().await?;
    let collection = db.collection::<T>(collection_name);

    let query = bson::doc! {"_id": _id};

    let result = collection.delete_one(query, None).await;
    result
}
