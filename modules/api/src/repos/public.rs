use salvo::prelude::*;
use crate::models::*;
use crate::middleware::redis_getter::RedisGetter;
use serde_json::Value;

#[handler]
pub async fn health_checker_handler(res: &mut Response, ctrl: &mut FlowCtrl) {
    res.render(Json(BasicResponse {status: "done".to_string(), message: "Health Checked!".to_string()}))
}

#[handler]
pub async fn teachers(res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) {
    let mut conn = depot.obtain::<RedisGetter>().unwrap();
    let list_of_teachers: String = conn.get_data("teachers");

    let mut json_object_of_teachers: Value = serde_json::from_str(&list_of_teachers).unwrap();
    println!("{}", list_of_teachers);
    res.render(Json(json_object_of_teachers));
}
