use actix_web::{ HttpResponse };


pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}
