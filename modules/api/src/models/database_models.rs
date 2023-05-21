use crate::models::abstract_models::*;
use serde::{ Deserialize, Serialize };
use mongodb::bson::{ self, oid::ObjectId };

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GroupDatabase {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub group_data: AbstractGroupData<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LessonDatabase {
    #[serde(rename="_id")]
    pub id: ObjectId,
    pub lesson_data: AbstractLessonData<String>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDatabase {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_data: AbstractUserData<String>,
    pub username: String,
    pub password: String,
    pub role: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ScheduleDatabase {
    #[serde(rename="_id")]
    pub id: ObjectId,
    pub group_id: String,
    pub date: String,
    pub lessons: Vec<AbstractLessonForScheduleData<i64, String>>
}
