use askama::Template;
use salvo::{writer::Text, handler, Response, Depot};

use crate::{configs::read_book_config::BookConfigs, SERVER_IP, SERVER_PORT};

use super::{Item, Items};


#[handler]
pub async fn home(res: &mut Response, depot: &Depot) {
    let config = depot.get::<BookConfigs>("config").unwrap();
    let mut items = Items { items: vec![] };
    let domain = format!("{}:{}", *SERVER_IP, *SERVER_PORT);
    for (k, _v) in config.configs.iter() {
        items.items.push(Item {
            title: k.clone(),
            path: {
                let book_name = k.clone();
                format!("{}/books/{}/book/index.html",domain, book_name)
            },
        })
    }
    items.items.sort_by_key(|m| m.title.clone());
    res.render(Text::Html(items.render().unwrap()))
}