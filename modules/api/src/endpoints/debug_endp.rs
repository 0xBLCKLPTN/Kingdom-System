use salvo::prelude::*;
use crate::middlewares::mongo_user_controller::MongoUserController;
use serde_json;
use crate::models::user_models::ResponseUser;



#[handler]
pub async fn get_users(res: &mut Response, req: &mut Request, depot: &mut Depot) {
    let mut database_connection = depot.obtain::<MongoUserController>();
    let result: Vec<ResponseUser> = database_connection.expect("Cant get users").get_all_users().await;
    res.render(Json(result))
}
