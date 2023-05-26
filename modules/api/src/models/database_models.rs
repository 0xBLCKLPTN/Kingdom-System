use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use crate::models::abstract_models::*;



#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LessonDatabase {
    #[serde(rename="_id")]
    pub id: ObjectId,
    pub lesson_data: LessonData<String>
}


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDatabase {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_data: UserData<String>,
    pub username: String,
    pub password: String,
}
