use salvo::prelude::*;
use crate::middlewares::mongo_schedule_controller::MongoScheduleController;
use crate::models::schedule_models::*;



#[handler]
pub async fn get_all_schedules(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot){
    let mut database_connection = depot.obtain::<MongoScheduleController>();
    let schedules: Vec<Schedule> = database_connection.expect("Cannot connect to database").get_schedules().await;

    res.render(Json(schedules))
}

#[handler]
pub async fn get_schedule_by(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) -> &'static str {
    "hello world!"
}

#[handler]
pub async fn add_schedule(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) -> &'static str {
    let mut database_connection = depot.obtain::<MongoScheduleController>();
    let schedule = req.parse_json::<NewSchedule>().await.unwrap();
    database_connection.expect("Cannot connect to database").add_schedule(schedule).await;
    "hello World!"
}

#[handler]
pub async fn delete_schedule(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) -> &'static str {
    let mut database_connection = depot.obtain::<MongoScheduleController>();
    let schedule = req.parse_json::<DeleteSchedule>().await.unwrap();
    database_connection.expect("Cannot connect to database").delete_schedule(&schedule.schedule_id).await;
    "Hello"
    
}
