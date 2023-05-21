use crate::models::abstract_models::AbstractTokenData;
use crate::models::database_models::{ GroupDatabase, LessonDatabase, ScheduleDatabase};
use serde::{ Deserialize, Serialize };
use mongodb::bson::{ self, oid::ObjectId };

type LessonResponse = LessonDatabase;
type ScheduleResponse = ScheduleDatabase;
type GroupResponse = GroupDatabase;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    #[serde(rename = "_id" )]
    pub id: ObjectId,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: AbstractTokenData<String, i64>,
    pub refresh_token: AbstractTokenData<String, i64>,
}
