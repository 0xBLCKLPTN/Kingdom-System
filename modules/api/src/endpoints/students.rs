use actix_web::{ HttpResponse };


pub async fn get_schedule() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

pub async fn get_lesson() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

pub async fn get_group() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}
