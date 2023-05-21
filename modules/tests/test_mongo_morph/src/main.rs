use mongodb::{ Client, Collection, Database};
use mongodb::options::{ ClientOptions, FindOptions };
use mongodb::bson::{ self, doc, to_document, Document };
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error as MongoError;

use serde::{ Deserialize, Serialize };
use std::str::FromStr;
use futures::TryStreamExt;
use std::fmt::Debug;

//#![feature(async_fn_in_trait)]

use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct MongoController<T> {
    collection: Collection<T>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,

    pub username: String,
    pub password: String,

    pub role: String,
}

pub async fn get_database() -> Result<Database, MongoError> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("Kingdom-Base".to_string());

    let client = Client::with_options(client_options)?;
    let database = client.database("Kingdom-Base");
    Ok(database)

}

pub async fn validate_user<T>(username: &str, password: &str, db: T) -> bool {
    let filter = doc! { "username": username, "password": password };
    match db.collection.find_one(Some(filter), None).await.unwrap() {
        Some(user) => return true,
        None => return false,
    }
}


impl<T> MongoController<T>
where T: Debug + Clone,
{
    pub async fn new(collection_name: &str) -> MongoController<T> {
        let database = get_database().await.unwrap();
        let collection = database.collection::<T>(collection_name);
        println!("✅ Mongo {} Controller connected successfully", collection_name);
        MongoController { collection }
    }

    pub async fn add_something<I>(&self, new_item: I) {
        println!("Hello!")
    }
    pub async fn get_something(&self, filter: Document, is_all: bool) {
        todo!();
    }
    pub async fn update_something<I>(&self, filter: Document, item: I) {
        todo!();
    }
    pub async fn delete_something(&self, filter: Document) {
        todo!();
    }
    pub async fn get_something_by(&self, filter: Document) {
        
    }

}

struct Point {
    pub x: i64,
    pub y: i64,
}

#[tokio::main]
async fn main() {
    let mc = MongoController::<User>::new("user_collection").await;
    mc.add_something(Point { x: 3, y: 3}).await;
    println!("Hello");
}
