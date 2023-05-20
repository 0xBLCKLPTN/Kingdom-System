use salvo::prelude::*;
use crate::models::user_models::{RoleChangingPayload};
use crate::middlewares::mongo_user_controller::MongoUserController;


#[handler]
pub async fn change_role(res: &mut Response, req: &mut Request, depot: &mut Depot) -> &'static str {
    let mut database_connection = depot.obtain::<MongoUserController>();
    let payload = req.parse_json::<RoleChangingPayload>().await.unwrap();
    database_connection.expect("Cannot connect to database").change_role(&payload.user_id, &payload.role).await;
    "Hello World!"
}


