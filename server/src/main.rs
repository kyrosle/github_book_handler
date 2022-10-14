use salvo::{cors::Cors, prelude::TcpListener, serve_static::static_embed, Router, Server};

use crate::service::{config_service::read_config, router_service::home, file_service::Assets};

use lazy_static::lazy_static;

mod bookhandler;
mod configs;
mod service;

pub struct ServerConfig {
}

lazy_static! {
    pub static ref SERVER_IP: &'static str = "localhost";
    pub static ref SERVER_PORT: &'static str = "7880";
}

#[tokio::main]
async fn main() {
    let domain = format!("{}:{}", *SERVER_IP, *SERVER_PORT);
    let cors_handler = Cors::builder()
        .allow_origin(domain.clone().as_ref())
        .allow_methods(vec!["GET"])
        .build();
    let router = Router::new().hoop(read_config).hoop(cors_handler).get(home);
    let router = router.push(
        Router::with_path("<**path>").get(static_embed::<Assets>().with_fallback("index.html")),
    );
    println!("Listening the | {} |", &domain);
    Server::new(TcpListener::bind(&domain))
        .serve(router)
        .await;
}
