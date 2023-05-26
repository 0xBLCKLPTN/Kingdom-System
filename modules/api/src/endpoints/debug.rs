use actix_web::{ web, web::{ Data, Json}, Result, Responder };
use crate::{
    models::{ request_payloads::*, response_payloads::*, },
    UserDatabase,
    MongoController,
};
use futures::TryStreamExt;

pub async fn get_all_users(db: Data<MongoController<UserDatabase>>) -> Result<impl Responder> {
    let mut cursor = db.collection.find(None, None).await.unwrap();
    let mut result: Vec<UserDatabase> = Vec::new();

    while let Some(user) = cursor.try_next().await.unwrap() {
        result.push(user)
    }
    Ok(Json(result))
}

