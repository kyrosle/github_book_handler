use salvo::{cors::Cors, prelude::TcpListener, serve_static::static_embed, Router, Server};
use server::{
    service::{config_service::read_config, file_service::Assets, router_service::home},
    SERVER_IP, SERVER_PORT,
};

pub struct ServerConfig {}

#[tokio::main]
async fn main() {
    // format domain form static settings from lib.rs
    let domain = format!("{}:{}", *SERVER_IP, *SERVER_PORT);

    // allow cors 
    let cors_handler = Cors::builder()
        .allow_origin(format!("http://{}:{}", *SERVER_IP, *SERVER_PORT).as_ref())
        .allow_methods(vec!["GET"])
        .build();
    
    // add middle ware allowing-cors and refresh read-config 
    let router = Router::new().hoop(cors_handler).hoop(read_config);

    // set file routers
    // "<**path>"
    let router = router.push(
        Router::with_path("<**path>").get(static_embed::<Assets>().with_fallback("index.html")),
    );
    // set home routers
    // "/"
    let router = router.push(
        Router::with_path("/").get(home),
    );

    println!("Listening the | {} |", &domain);
    // bind listener
    let listener = TcpListener::bind(&domain);
    // startup server
    Server::new(listener).serve(router).await;
}
