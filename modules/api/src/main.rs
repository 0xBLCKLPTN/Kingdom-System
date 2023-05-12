mod models;
mod repos;
mod middleware;

use salvo::affix;
use models::TokenClaims;
use salvo::prelude::*;
use repos::{
    public::{
        health_checker_handler,
        teachers,
    },
    operator::{
        set_new_teacher,
        edit_teacher,
    },
    auth::{login, self},
};
use jsonwebtoken::{self, EncodingKey};
use salvo::http::{Method, StatusError};
use salvo::jwt_auth::QueryFinder;
use middleware::redis_getter;

const SECRET_KEY: &str = "MYSUPERSECRETKEY";

fn auth_handler() -> JwtAuth<TokenClaims> {
    JwtAuth::new(SECRET_KEY.to_owned())
        .finders(vec![
            Box::new(QueryFinder::new("jwt_token")),
        ])
        .response_error(true) // if true then current page not work.
}

fn router_creator() -> Router {
    Router::new()
        .get(health_checker_handler)
        .push(
            Router::with_path("auth")
            .push(
                Router::with_path("login")
                    .handle(login)
            )
            .push(
                Router::with_path("register")
                    .get(login)
            )
        )
        .push(
            Router::with_path("api")
                .push(
                    Router::with_path("public")
                        .push(
                            Router::with_path("teachers")
                                .hoop(affix::inject(redis_getter::RedisGetter::new()))
                                .get(teachers)
                        )
                )
                .push(
                    Router::with_path("operator")
                    .hoop(auth_handler())
                    .hoop(
                        affix::inject(redis_getter::RedisGetter::new())
                    )    
                    .push(
                        Router::with_path("set_teacher")    
                        .post(set_new_teacher)
                    )
                    .push(
                        Router::with_path("edit-teacher")
                        .post(edit_teacher)
                    )
                )
        )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router_creator()).await;
}
