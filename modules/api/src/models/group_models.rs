use serde::{Deserialize, Serialize};
use mongodb::bson::{self, oid::ObjectId};


#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Group {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub group_short_name: String,
    pub group_name: String,
    pub main_teacher_id: String,
    pub course: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewGroup {
    pub group_short_name: String,
    pub group_name: String,
    pub main_teacher_id: String,
    pub course: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteGroup {
    pub group_id: String,
}
