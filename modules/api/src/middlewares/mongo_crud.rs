use mongodb::{
    Client,
    options::{
        ClientOptions, FindOptions,
    },
    Collection,
    error::Error as ER
};
use mongodb::bson::{self, oid::ObjectId, doc, to_document};
use serde::{Deserialize, Serialize};
use crate::models::NewUser;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: i64,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct MongoController {
    user_collection: Collection<User>,
}


impl MongoController {
    pub async fn new() -> Result<Self, ER> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("Kingdom-UserBase".to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database("Kingdom-UserBase");

        let user_collection = database.collection::<User>("users_collection");

        println!("✅ Database connected successfully");
        Ok(Self { user_collection })
    }
    
    pub async fn validate_user(&self, username: &str, password: &str) -> bool {
        let filter = doc! { "username": username, "password": password };
        let result = match self.user_collection.find_one(Some(filter), None).await.unwrap() {
            Some(user) => return true,
            None => return false,
        };
    }

    pub async fn register_user(&self, user: NewUser) -> bool {
        let mut bson_doc = doc! {
            "id": 1
            "first_name": "qwe",
            "middle_name": "qwe",
            "last_name": "wer",
            "username": "dfg",
            "password": "gdfg"
        };
        let result = self.user_collection.insert_one(bson_doc, None).await.unwrap();
        true

    }
}
