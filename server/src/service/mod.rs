use askama::Template;

pub mod file_service;
pub mod router_service;
pub mod config_service;

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