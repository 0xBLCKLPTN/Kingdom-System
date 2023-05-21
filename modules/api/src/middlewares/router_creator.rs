use salvo::prelude::Router;


pub async fn auth_router() -> Router {
    Router::with_path("auth")
        .push(Router::with_path("sign-in").post())
        .push(Router::with_path("sign-up").post())
}

pub async fn debug_router() -> Router {
    Router::with_path("debug")
        .push(Router::with_path("get-users").get())
}

pub async fn teachers_router() {
    todo!();
}

pub async fn operator_router() -> Router {
    Router::with_path("operator")
        .push(
            Router::with_path("control-users")
                .push(Router::with_path("user").delete().put().put())
        ).push(
            Router::with_path("control-groups")
                .push(Router::with_path("group").delete().put().post())
        ).push(
            Router::with_path("control-lessons")
                .push(Router::with_path("lesson").delete().put().post())
        ).push(
            Router::with_path("control-schedules")
                .push(Router::with_path("schedule").delete().put().post())
        )
}

pub async fn student_router() -> Router {
    Router::with_path("student")
        .push(Router::with_path("get-teachers").get())
        .push(Router::with_path("get-lessons").get())
        .push(Router::with_path("get-schedule").get())
        .push(Router::with_path("get-groups").get())
}

pub async fn main_router() -> Router {
    Router::new()
        .push(student_router().await)
        .push(auth_router().await)
        .push(debug_router().await)
        .push(operator_router().await)
}
