/*
    teachers.rs

Teachers handlers for KS-api. This handlers only for `/teachers` router.
Project: Kingdom-System-api
Authod: blcklptn

*/

use actix_web::{ web, get, HttpResponse, Responder };


// http://127.0.0.1:8080/api/teachers
#[get("/")]
pub async fn teachers() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World!"))
}

// http://127.0.0.1:8080/api/teacher?id=1
#[get("/teacher")]
pub async fn teacher() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World!"))
}
