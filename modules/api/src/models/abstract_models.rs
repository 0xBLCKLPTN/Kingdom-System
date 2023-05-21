use serde::{ Deserialize, Serialize };

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractScheduleData<S> {
    pub group_id: S,
    pub date: S,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractUserData<S> {
    pub first_name: S,
    pub middle_name: Option<S>,
    pub last_name: S,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractLessonData<S> {
    pub teacher_id: S,
    pub lesson_name: S,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractLessonForScheduleData<I> {
    pub lesson_number: I,
    pub lesson_data: AbstractLessonData<String>,
}


#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractGroupData<S> {
    pub main_teacher_id: S,
    pub group_name: S,
    pub group_short_name: Option<S>,
    pub course: S,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbstractTokenData<S, I> {
    pub token: S,
    pub exp: I,
}
