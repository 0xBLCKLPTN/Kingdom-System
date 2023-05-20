use serde::{Deserialize, Serialize};
use mongodb::bson::{self, oid::ObjectId};


#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Lesson {

    #[serde(rename="_id")]
    pub id: ObjectId,
    pub teacher_id: String,
    pub lesson_name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LessonForSchedule {

    pub lesson_number: i64,
    pub teacher_id: String,
    pub lesson_name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewLesson {
    pub teacher_id: String,
    pub lesson_name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteLesson {
    pub lesson_id: String,
}