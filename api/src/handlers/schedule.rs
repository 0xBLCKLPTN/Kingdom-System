/*
    schedule.rs

Schedule handlers for KS-api. This handlers only for `/api/schedules` router.
Project: Kingdom-System-api
Authod: blcklptn

*/

use actix_web::{ web, HttpResponse, get, Responder };


// http://127.0.0.1:8080/api/schedules/
#[get("/")]
pub async fn schedules() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World!"))
}

// http://127.0.0.1:8080/api/schedules/group?id=1
#[get("/group")]
pub async fn schedule_by_group_id() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World!"))
}

// http://127.0.0.1:8080/api/schedules/teacher?id=1
#[get("/teacher")]
pub async fn schedule_by_teacher_id() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World!"))
}
