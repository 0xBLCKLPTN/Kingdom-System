use serde::{ Deserialize, Serialize };


#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LessonData<S> {
    pub teacher_id: S,
    pub lesson_name: S, 
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserCredentials<S> {
    pub username: S,
    pub password: S,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserData<S> {
    pub first_name: S,
    pub middle_name: Option<S>,
    pub last_name: S
}
