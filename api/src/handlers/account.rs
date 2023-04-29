/*
    account.rs

Authefication handlers for KS-api. This handlers only for `/account` router.
Project: Kingdom-System-api
Authod: blcklptn

*/

use actix_web::{ web, post, HttpResponse, Responder };

// http://127.0.0.1:8080/account/signIn
#[post("/signIn")]
pub async fn sign_in() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}


// http://127.0.0.1:8080/account/signUp
#[post("/signUp")]
pub async fn sign_up() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}
