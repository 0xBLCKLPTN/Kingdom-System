use mongodb::{
    Client,
    options::{
        ClientOptions, FindOptions,
    },
    Collection,
    error::Error as ER
};
use mongodb::bson::{self, oid::ObjectId, doc, to_document, Document};
use serde::{Deserialize, Serialize};
use crate::models::user_models::{User, NewUser, ResponseUser};
use std::str::FromStr;
use futures::TryStreamExt;


#[derive(Clone, Debug)]
pub struct MongoUserController {
    user_collection: Collection<User>,
}


impl MongoUserController {
    pub async fn new() -> Result<Self, ER> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("Kingdom-Base".to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database("Kingdom-Base");

        let mut user_collection = database.collection::<User>("users_collection");
        
        println!("✅ Mongo User Controller connected successfully");
        Ok(Self { user_collection })
    }
    
    pub async fn validate_user(&self, username: &str, password: &str) -> bool {
        let filter = doc! { "username": username, "password": password };
        let result = match self.user_collection.find_one(Some(filter), None).await.unwrap() {
            Some(user) => return true,
            None => return false,
        };
    }

    pub async fn check_on_user_exists(&self, username: &str) -> bool {
        let filter = doc! { "username": username};
        let result = match self.user_collection.find_one(Some(filter), None).await.unwrap() {
            Some(user) => return true,
            None => return false,
        };
    }

    pub async fn get_all_users(&self) -> Vec<ResponseUser> {
        let mut cursor = self.user_collection.find(None, None).await.unwrap();
        let mut result: Vec<ResponseUser> = Vec::new();
        while let Some(user) = cursor.try_next().await.unwrap() {
            result.push(
                ResponseUser {
                    id: user.id,
                    first_name: user.first_name,
                    middle_name: user.middle_name,
                    last_name: user.last_name,
                }
            ); 
        }
        result
    }

    pub async fn register_user(&self, new_user: NewUser) -> bool {
        if self.check_on_user_exists(&new_user.username).await {
            return false;
        }
        let user = User {
            id: ObjectId::new(),
            first_name: new_user.first_name.clone(),
            middle_name: new_user.middle_name.clone(),
            last_name: new_user.last_name.clone(),
            username: new_user.username.clone(),
            password: new_user.password.clone(),
            role: "student".to_string(),

        };
        self.user_collection.insert_one(user, None).await.unwrap();
        return true;

    }

    pub async fn get_users_by_role(&self, role: &str) -> Vec<ResponseUser>{
        let mut cursor = self.user_collection.find(Some(doc! {"role": role}), None).await.unwrap();
        let mut result: Vec<ResponseUser> = Vec::new();
        while let Some(user) = cursor.try_next().await.unwrap() {
            result.push(
                ResponseUser {
                    id: user.id,
                    first_name: user.first_name,
                    middle_name: user.middle_name,
                    last_name: user.last_name,
                }
            );
        }
        result
    }

    pub async fn change_role(&self, user_id: &str, role: &str){
        let filter = doc! {"_id": ObjectId::parse_str(user_id).unwrap()};
        let update = doc! {"$set": {"role": role}};
        self.user_collection.update_one(filter, update, None).await.unwrap();
    }

    pub async fn delete_user(&self, user_id: &str) -> anyhow::Result<()> {
        let filter = doc! {"_id": ObjectId::parse_str(user_id).unwrap()};
        self.user_collection.delete_one(filter, None).await;
        Ok(())
    }
}
