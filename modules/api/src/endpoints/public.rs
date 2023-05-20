use salvo::prelude::*;
use crate::models::{BasicResponse};

#[handler]
pub async fn health_checker(res: &mut Response, ctrl: &mut FlowCtrl) {
    res.render(
        Json(
            BasicResponse { status: "done".to_string(), message: "Health Checked!".to_string() }
        )
    )
}

#[handler]
pub async fn get_all_schedule(res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) -> &'static str {
    "hello World!"
}

#[handler]
pub async fn get_schedule_by(res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) -> &'static str {
    "hello world!"
} 
