use salvo::prelude::*;

#[handler]
pub async fn get_all_schedules(res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) -> &'static str {
    "hello World!"
}

#[handler]
pub async fn get_schedule_by(res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) -> &'static str {
    "hello world!"
}
