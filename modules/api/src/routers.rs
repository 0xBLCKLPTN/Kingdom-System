use actix_web::web;

#[path="./handlers/mod.rs"]
pub mod handlers;
use handlers::{account, groups, operator, public, schedule, teachers, technics};



// Routers for KS-api project.
pub fn api( cfg: &mut web::ServiceConfig ) {
    cfg
        // http://127.0.0.1:8080/account/
        .service(web::scope("/account")
                .service(account::sign_in)
                .service(account::sign_up)
        )
        // http://127.0.0.1:8080/api
       .service(web::scope("/api")
                  .service(groups::groups)
                  .service(groups::group)
                  .service(teachers::teachers)
                  .service(teachers::teacher)

                  // http://127.0.0.1:8080/api/schedules
                  .service(
                      web::scope("/schedules")
                        .service(schedule::schedules)
                        .service(schedule::schedule_by_group_id)
                        .service(schedule::schedule_by_teacher_id)
                )
        );

}
