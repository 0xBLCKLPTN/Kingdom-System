mod router_creator;

pub mod middlewares;
pub mod endpoints;
pub mod models;

use salvo::prelude::*;
use router_creator::generate_default_router;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(generate_default_router().await).await;
}

