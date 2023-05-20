use crate::models::token_models::TokenClaims;
use crate::middlewares::{
    mongo_user_controller::MongoUserController,
    mongo_group_controller::MongoGroupController,
    mongo_lesson_controller::MongoLessonController,
    mongo_schedule_controller::MongoScheduleController,
};
use crate::endpoints::{
    public::health_checker,
    teachers::{
        get_all_teachers, get_teacher_by,
    },
    schedules::{
        get_all_schedules, get_schedule_by, add_schedule, delete_schedule,
    },
    auth::{
        login, register
    },
    debug_endp::{
        get_users,
    },
    operator::{
       change_role, 
    },
    groups::{
        add_group,
        delete_group,
        get_groups,
    },
    lessons::{
        get_lessons,
        delete_lesson,
        add_lesson,
    },
};

use jsonwebtoken::{self, EncodingKey};
use salvo::prelude::*;
use salvo::http::{Method, StatusError};
use salvo::jwt_auth::QueryFinder;
use salvo::affix;

const SECRET_KEY: &str = "MYSUPERSECRETKEY";

fn auth_handler() -> JwtAuth<TokenClaims> {
    JwtAuth::new(SECRET_KEY.to_owned())
        .finders(vec![Box::new(QueryFinder::new("jwt_token")),])
        .response_error(true)
}

pub async fn generate_default_router() -> Router {
    Router::new()
        .hoop(affix::inject(MongoUserController::new().await.unwrap()))
        .hoop(affix::inject(MongoGroupController::new().await.unwrap()))
        .hoop(affix::inject(MongoLessonController::new().await.unwrap()))
        .hoop(affix::inject(MongoScheduleController::new().await.unwrap()))
        .get(health_checker)
        .push(
            Router::with_path("debug")
                .hoop(auth_handler())
                .push(Router::with_path("get-users").get(get_users))
        )
        .push(
            Router::with_path("auth")
                .push(Router::with_path("login").handle(login))
                .push(Router::with_path("register").post(register))
            )
            .push(
                Router::with_path("api")
                    .push(
                        Router::with_path("public")
                        .push(
                            Router::with_path("groups")
                                .get(get_groups)
                        )
                        .push(
                            Router::with_path("lessons")
                                .get(get_lessons)
                        )
                        .push(
                                Router::with_path("teachers")
                                    .get(get_all_teachers)
                                    .push(Router::with_path("by").get(get_teacher_by))
                            )
                        .push(
                            Router::with_path("schedule")
                                .get(get_all_schedules)
                                .push(Router::with_path("by").get(get_schedule_by))
                    )
                    

                    )
                    .push(
                        Router::with_path("operator")
                            .hoop(auth_handler())
                            .push(Router::with_path("change-role").post(change_role))
                            .push(
                                Router::with_path("groups-management")
                                    .push(
                                        Router::with_path("add-group")
                                        .post(add_group)
                                    )
                                    .push(
                                        Router::with_path("delete-group")
                                        .post(delete_group)
                                    )
                            )
                            .push(
                                Router::with_path("lesson-management")
                                    .push(
                                        Router::with_path("add-lesson")
                                        .post(add_lesson)
                                    )
                                    .push(
                                        Router::with_path("delete-lesson")
                                        .post(delete_lesson)
                                    )
                            )
                            .push(
                                Router::with_path("user-management").get(health_checker)
                            )
                            .push(
                                Router::with_path("schedule-management")
                                    .push(
                                        Router::with_path("add-schedule")
                                        .post(add_schedule)
                                    )
                                    .push(
                                        Router::with_path("delete-schedule")
                                        .post(delete_schedule)
                                    )
                            )
                            
                        )
            )
}
