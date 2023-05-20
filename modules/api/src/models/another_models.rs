use serde::{Deserialize, Serialize};
use mongodb::bson::{self, oid::ObjectId};


#[derive(Deserialize, Serialize, Debug)]
pub struct AuthenticateCredentials {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize)]
pub struct BasicResponse {
    pub status: String,
    pub message: String,
}
