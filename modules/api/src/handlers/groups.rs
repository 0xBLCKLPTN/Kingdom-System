/*
    groups.rs

Groups handlers for KS-api. This handlers only for `/api` router.
Project: Kingdom-System-api
Authod: blcklptn

*/

use actix_web::{ web, get, HttpResponse, Responder };

// http://127.0.0.1:8080/api/groups
#[get("/groups")]
pub async fn groups() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

// http://127.0.0.1:8080/api/group?id=1
#[get("/group")]
pub async fn group() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}
