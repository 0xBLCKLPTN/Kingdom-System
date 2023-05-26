use actix_web::{ web, web::{ Data, Json}, Result, Responder };
use crate::{
    models::{ request_payloads::*, response_payloads::*, },
    UserDatabase,
    MongoController,
};

use mongodb::bson::{oid::ObjectId};

pub async fn sign_in(db: Data<MongoController<UserDatabase>>, credentials: Json<SignInCredentials>) -> Result<impl Responder> {
    Ok(web::Json(DefaultResponse { status: "Done!", message: "Welcome back!"}))
}

pub async fn sign_up(db: Data<MongoController<UserDatabase>>, credentials: Json<SignUpCredentials>) -> Result<String> {
    let user = UserDatabase {
        id: ObjectId::new(),
        user_data: credentials.0.user_data,
        username: credentials.0.credentials.username,
        password: credentials.0.credentials.password,
    };
    db.collection.insert_one(user, None).await.unwrap();
    Ok(format!("Welcome Hellor!"))
}
