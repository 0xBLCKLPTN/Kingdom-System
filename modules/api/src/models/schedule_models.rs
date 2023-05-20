use serde::{Deserialize, Serialize};
use mongodb::bson::{self, oid::ObjectId};
use crate::models::lesson_models::{NewLesson, Lesson, LessonForSchedule};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewSchedule {
    pub group_id: String,
    pub date: String,
    pub lessons: Vec<LessonForSchedule>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Schedule {
    #[serde(rename="_id")]
    pub id: ObjectId,
    pub group_id: String,
    pub date: String,
    pub lessons: Vec<LessonForSchedule>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteSchedule {
    pub schedule_id: String
}