use salvo::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct NewTeacher {
    pub firstName: String,
    pub middleName: String,
    pub lastName: String,
}

#[handler]
pub async fn set_new_teacher(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) {
    let teacher: NewTeacher = req.parse_json::<NewTeacher>().await.unwrap();
    res.render(Json(teacher));
}

#[handler]
pub async fn edit_teacher(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) -> &'static str {
    let teacher_id: String = req.query("teacher-id").unwrap();
    let teacher_new_data: NewTeacher = req.parse_json::<NewTeacher>().await.unwrap();
    println!("{}", teacher_id);
    "Hello World!"
}
