use salvo::prelude::*;
use crate::models::*;

#[handler]
pub async fn health_checker_handler(res: &mut Response, ctrl: &mut FlowCtrl) {
    res.render(Json(BasicResponse {status: "done".to_string(), message: "Health Checked!".to_string()}))
}

#[handler]
pub async fn teachers(res: &mut Response, ctrl: &mut FlowCtrl) {
    res.render(Json(Teacher { name: "Daniil".to_string() }))
}