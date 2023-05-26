/* File: main.rs
The starting point of the entire system. 
This is where the application is configured and started with the Actix framework.
Actix was not chosen by chance, but because it is faster, even though it is harder 
to write on than the salvo framework.

Author: 0xblcklptn;
LICENSE: GPL-3.0;
*/

pub mod middlewares;
pub mod endpoints;
pub mod config;
pub mod models;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use middlewares::{ router_creator, controller::MongoController };

use actix_web::middleware::Logger;
use env_logger::Env;
use structopt::StructOpt;

use models::database_models::{UserDatabase, LessonDatabase};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer, web::Data};

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let user_db = MongoController::<UserDatabase>::new("user_collection").await.unwrap();
    let lesson_db = MongoController::<LessonDatabase>::new("lesson_collection").await.unwrap();
    
    let opt = config::Opt::from_args();
    let api_config: config::Config = config::configure(&opt.conf).unwrap();    

    HttpServer::new(move ||
        App::new()
        .app_data(Data::new(user_db.clone()))
        .app_data(Data::new(lesson_db.clone()))
        .wrap(Logger::default())
        .configure(router_creator::default_router)
    ).bind(api_config.host_url)?.run().await
}
