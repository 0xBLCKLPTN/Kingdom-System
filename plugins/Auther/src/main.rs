use actix_web::{web, App, HttpResponse, Responder, get, post, HttpServer};
use env_logger::Env;
use actix_web::middleware::Logger;


#[get("/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/api/SignUn")]
async fn post_sign_up() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/api/SignIn")]
async fn post_sign_in() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(get_health)
            .service(post_sign_in)
            .service(post_sign_up)
    }).bind(("127.0.0.1", 8080))?.run().await
}
