use actix_web::{get, web, HttpResponse, Responder};


// http://127.0.0.1:8080/
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World!"))
}
