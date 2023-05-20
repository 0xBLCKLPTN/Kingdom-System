mod router_creator;
mod middlewares;

pub mod endpoints;
pub mod models;

use middlewares::mongo_crud;
use salvo::prelude::*;
use router_creator::generate_default_router;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(generate_default_router()).await;
}

