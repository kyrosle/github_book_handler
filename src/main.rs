use crate::configs::Configs;
use askama::Template;
use bookhandler::{download_github, get_book_resources_path};
use rust_embed::RustEmbed;
use salvo::{
    cors::Cors,
    prelude::*,
    serve_static::{static_embed, EmbeddedFileExt},
};

mod bookhandler;
mod configs;

#[derive(Template, Debug)]
#[template(path = "index.html")]
struct Items {
    pub items: Vec<Item>,
}

#[derive(Debug)]
struct Item {
    pub title: String,
    pub path: String,
}
#[handler]
async fn read_config(depot: &mut Depot) {
    let config = Configs::new_init().unwrap_or_else(|_| {
        println!("can't read configs/config.yml !! \n using empty default");
        Configs::default()
    });
    for (k, v) in config.configs.iter() {
        if !v.local {
            match download_github(k, v.path.clone().unwrap().as_ref()) {
                Ok(_) => {}
                Err(e) => {
                    println!("error git clone: {}", e);
                }
            }
        }
    }
    depot.insert("config", config);
}

#[handler]
async fn home(res: &mut Response, depot: &Depot) {
    let config = depot.get::<Configs>("config").unwrap();
    let mut items = Items { items: vec![] };
    for (k, _v) in config.configs.iter() {
        items.items.push(Item {
            title: k.clone(),
            path: {
                let book_name = k.clone();
                format!("http://localhost:8080/books/{}/book/index.html", book_name)
            },
        })
    }
    items.items.sort_by_key(|m| m.title.clone());
    res.render(Text::Html(items.render().unwrap()))
}

#[derive(RustEmbed)]
#[folder = "resources"]
struct Assets;

#[tokio::main]
async fn main() {
    let cors_handler = Cors::builder()
        .allow_origin("http://localhost:8080")
        .allow_methods(vec!["GET"])
        .build();
    let router = Router::new().hoop(read_config).hoop(cors_handler).get(home);
    let router = router.push(
        Router::with_path("<**path>").get(static_embed::<Assets>().with_fallback("index.html")),
    );
    println!("Listening the | http://localhost:8080 |");
    Server::new(TcpListener::bind("localhost:8080"))
        .serve(router)
        .await;
}
