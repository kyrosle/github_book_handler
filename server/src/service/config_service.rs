use salvo::{handler, Depot};

use crate::{
    bookhandler::download::download_github,
    configs::{read_book_config::BookConfigs, ReadConfig},
};

/// Read the book-config from configs/book-config.yml
///
/// insert `book-config` to the depot for subsequent calls
#[handler]
pub async fn read_config(depot: &mut Depot) {
    // from BookConfigs data-struct
    // deserialize from book-config.yml
    let config = BookConfigs::new_init().unwrap_or_else(|_| {
        println!("can't read configs/book-config.yml !! \n using empty default");
        BookConfigs::default()
    });
    // handle the books data from configs
    for (k, v) in config.configs.iter() {
        // wether it's using local book
        // or else it should download it from github.com  or other urls
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
