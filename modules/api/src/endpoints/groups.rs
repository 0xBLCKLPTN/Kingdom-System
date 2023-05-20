use salvo::prelude::*;
use crate::middlewares::mongo_group_controller::MongoGroupController;
use crate::models::group_models::{NewGroup, DeleteGroup, Group};

#[handler]
pub async fn add_group(res: &mut Response, req: &mut Request, depot: &mut Depot) -> &'static str {
    let mut database_connection = depot.obtain::<MongoGroupController>();
    let group = req.parse_json::<NewGroup>().await.unwrap();
    database_connection.expect("Cannot connect to database").new_group(group).await;
    "hello World!"
}

#[handler]
pub async fn delete_group(res: &mut Response, req: &mut Request, depot: &mut Depot) {
    let mut database_connection = depot.obtain::<MongoGroupController>();
    let group = req.parse_json::<DeleteGroup>().await.unwrap();
    database_connection.expect("Cannot connect to database").delete_group(&group.group_id).await;
}

#[handler]
pub async fn get_groups(res: &mut Response, req: &mut Request, depot: &mut Depot) {
    let mut database_connection = depot.obtain::<MongoGroupController>();
    let groups: Vec<Group> = database_connection.expect("Cannot connect to database").get_groups().await;
    res.render(Json(groups))
}

