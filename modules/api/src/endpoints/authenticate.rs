use actix_web::{ web, web::{ Data, Json}, Result, Responder };
use mongodb::bson::{oid::ObjectId, doc, Document};
use crate::{
    models::{ request_payloads::*, response_payloads::*, },
    UserDatabase,
    MongoController,
};


pub async fn sign_in(db: Data<MongoController<UserDatabase>>, credentials: Json<SignInCredentials>) -> Result<impl Responder> {
    let filter = doc! { "username": credentials.0.username, "password": credentials.0.password };
    match db.collection.find_one(Some(filter), None).await.unwrap() {
        Some(user) => return Ok(web::Json(DefaultResponse { status: "Done!", message: "Welcome back!"})),
        None => return Ok(web::Json(DefaultResponse { status: "Error!", message: "authentication error"})),
    }
}

pub async fn sign_up(db: Data<MongoController<UserDatabase>>, credentials: Json<SignUpCredentials>) -> Result<String> {
    let filter = doc! { "username": &credentials.0.credentials.username };
    
    let result = match db.collection.find_one(Some(filter), None).await.unwrap() {
        Some(user) => return Ok(format!("Username already has been taken")),
        None => false,
    };

    let user = UserDatabase {
        id: ObjectId::new(),
        user_data: credentials.0.user_data,
        username: credentials.0.credentials.username,
        password: credentials.0.credentials.password,
    };
    db.collection.insert_one(user, None).await.unwrap();
    Ok(format!("Welcome!"))
}
