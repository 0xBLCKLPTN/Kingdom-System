use crate::models::{BasicResponse};
use salvo::prelude::*;


#[handler]
pub async fn get_all_teachers(res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) {
    res.render(
        Json(
            BasicResponse { status: "teachers".to_string(), message: "Teacher 1, Teacher2".to_string() }
        )
    )
}

#[handler]
pub async fn get_teacher_by(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) -> &'static str {
    let requered_teacher_id = req.query::<String>("id").unwrap();
    "Hello World!"
}

