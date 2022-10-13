use rbatis;
use rbatis::{crud, impl_select, Rbatis};
use rbdc_pg::driver::PgDriver;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: usize,
    pub name: String,
    pub info: Option<String>,
}

crud!(Person{});

#[tokio::main]
async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = Rbatis::new();
    rb.init(PgDriver{}, "postgres://postgres:1234@localhost:55435/database").unwrap();
    let mut tx = rb.acquire_begin().await.unwrap();

    let res = Person::select_all(&mut tx).await.unwrap();
    println!("Res : \n{:#?}", res);

    tx.commit().await.unwrap();
    tx.rollback().await.unwrap();
}