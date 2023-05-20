
use crate::endpoints::{
    public::health_checker,
    teachers::{
        get_all_teachers,
        get_teacher_by,
    },
    schedules::{
        get_all_schedules,
        get_schedule_by,
    },
    auth::{
        login,
    }
};
use salvo::prelude::*;
use jsonwebtoken::{self, EncodingKey};
use salvo::http::{Method, StatusError};
use salvo::jwt_auth::QueryFinder;
use crate::models::TokenClaims;

const SECRET_KEY: &str = "MYSUPERSECRETKEY";

fn auth_handler() -> JwtAuth<TokenClaims> {
    JwtAuth::new(SECRET_KEY.to_owned())
        .finders(vec![Box::new(QueryFinder::new("jwt_token")),])
        .response_error(true)
}

pub fn generate_default_router() -> Router {
    Router::new()
        .get(health_checker)
        .push(
            Router::with_path("auth")
                .push(Router::with_path("login").handle(login))
                .push(Router::with_path("register"))
            )
            .push(
                Router::with_path("api")
                    .push(
                        Router::with_path("public")
                            .push(
                                Router::with_path("teachers")
                                    .get(get_all_teachers)
                                    .push(Router::with_path("by").get(get_teacher_by))
                            )
                    )
                    .push(
                        Router::with_path("schedule")
                            .get(get_all_schedules)
                            .push(Router::with_path("by").get(get_schedule_by))

                    )
            )
}
