use crate::models::abstract_models::*;
use serde::{ Deserialize, Serialize };

// <----------------------------- [ User Structs ]
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InsertUser {
    pub user_data: AbstractUserData,
    pub username: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateUser {
    pub user_id: String,
    pub new_user_data: AbstractUserData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteUser {
    pub user_id: String,
}

// <----------------------------- [ Group Structs ]
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InsertGroup {
    pub group_data: AbstractGroupData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteGroup {
    pub group_data: AbstractGroupData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateGroup {
    pub group_id: String,
    pub new_group_data: AbstractGroupData,
}

// <----------------------------- [ Schedule Structs ]
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InsertSchedule {
    pub schedule_data: AbstractScheduleData,
    pub lessons: Vec<AbstractLessonForScheduleData>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteSchedule {
    pub schedule_id: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateSchedule {
    pub schedule_id: String,
    pub new_schedule_data: AbstractScheduleData,
}

// <----------------------------- [ Token Structs ]
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenClaims {
    pub username: String,
    pub exp: i64,
}



