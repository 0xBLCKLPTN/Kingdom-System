use serde::{Deserialize, Serialize};
use mongodb::bson::{self, oid::ObjectId};


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

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct NewUser {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,

    pub username: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RoleChangingPayload {
    pub user_id: String,
    pub role: String,
}