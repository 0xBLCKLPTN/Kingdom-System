use actix_web::web;
use crate::endpoints::{
    public::{health},
    authenticate::{sign_in, sign_up},
    students::{get_lesson, get_group, get_schedule},
    operator::{add_lesson, add_group, add_schedule},
    debug::{get_all_users},
};

fn authenticate_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/authenticate")
            .service(web::resource("/sign-in").route(web::post().to(sign_in)))
            .service(web::resource("/sign-up").route(web::post().to(sign_up)))
    );
}


fn public(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/public")
            .service(web::resource("/health").route(web::get().to(health)))
    );
}

fn student(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/student")
            .service(web::resource("/groups").route(web::get().to(get_group)))
            .service(web::resource("/schedule").route(web::get().to(get_schedule)))
            .service(web::resource("/lesson").route(web::get().to(get_lesson)))
    );
}

fn operator(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/operator")
            .service(web::resource("/groups").route(web::get().to(add_group)))
            .service(web::resource("/schedule").route(web::get().to(add_schedule)))
            .service(web::resource("/lesson").route(web::get().to(add_lesson)))
    );
}

fn debug_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/debug")
            .service(web::resource("get-all-users").route(web::get().to(get_all_users)))
    );
}

fn v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
        .configure(authenticate_router)
        .configure(student)
        .configure(operator)
        .configure(debug_router)
        .configure(public)
    );
}


pub fn default_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").configure(v1)
    );
}
