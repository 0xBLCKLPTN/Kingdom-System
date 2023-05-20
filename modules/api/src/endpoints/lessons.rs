use salvo::prelude::*;
use crate::middlewares::mongo_lesson_controller::MongoLessonController;
use crate::models::lesson_models::{NewLesson, Lesson, DeleteLesson};

#[handler]
pub async fn add_lesson(res: &mut Response, req: &mut Request, depot: &mut Depot) -> &'static str {
    let mut database_connection = depot.obtain::<MongoLessonController>();
    let lesson = req.parse_json::<NewLesson>().await.unwrap();
    database_connection.expect("Cannot connect to database").add_lesson(lesson).await;
    "hello World!"
}

#[handler]
pub async fn delete_lesson(res: &mut Response, req: &mut Request, depot: &mut Depot) {
    let mut database_connection = depot.obtain::<MongoLessonController>();
    let lesson = req.parse_json::<DeleteLesson>().await.unwrap();
    database_connection.expect("Cannot connect to database").delete_lesson(&lesson.lesson_id).await;
}

#[handler]
pub async fn get_lessons(res: &mut Response, req: &mut Request, depot: &mut Depot) {
    let mut database_connection = depot.obtain::<MongoLessonController>();
    let lessons: Vec<Lesson> = database_connection.expect("Cannot connect to database").get_lessons().await;

    res.render(Json(lessons))
}

