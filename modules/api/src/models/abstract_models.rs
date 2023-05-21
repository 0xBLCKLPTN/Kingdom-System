use serde::{ Deserialize, Serialize };

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractScheduleData {
    pub group_id: String,
    pub date: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractUserData {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractLessonData {
    pub teacher_id: String,
    pub lesson_name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractLessonForScheduleData {
    pub lesson_number: i64,
    pub lesson_data: AbstractLessonData,
}


#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractGroupData {
    pub main_teacher_id: String,
    pub group_name: String,
    pub group_short_name: Option<String>,
    pub course: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractTokenData {
    pub token: String,
    pub exp: i64,
}
