use crate::models::{
    another_models::BasicResponse,
    user_models::ResponseUser
};
use crate::middlewares::mongo_user_controller::MongoUserController;
use salvo::prelude::*;


#[handler]
pub async fn get_all_teachers(res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) {
    let mut database_connection = depot.obtain::<MongoUserController>();
    let teachers: Vec<ResponseUser> = database_connection.expect("Cannot connect to database").get_users_by_role("teacher").await;

    res.render(Json(teachers))
}

#[handler]
pub async fn get_teacher_by(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) -> &'static str {
    let requered_teacher_id = req.query::<String>("id").unwrap();
    "Hello World!"
}

