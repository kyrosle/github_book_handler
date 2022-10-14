use salvo::{handler, Depot};

use crate::{
    bookhandler::download_github,
    configs::{read_book_config::BookConfigs, ReadConfig},
};

#[handler]
pub async fn read_config(depot: &mut Depot) {
    let config = BookConfigs::new_init().unwrap_or_else(|_| {
        println!("can't read configs/book-config.yml !! \n using empty default");
        BookConfigs::default()
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
