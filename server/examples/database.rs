use rbatis::{crud, Rbatis};
use rbdc_pg::driver::PgDriver;
use rbs::to_value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: usize,
    pub name: String,
    pub info: Option<String>,
}

crud!(Person {});

#[tokio::main]
async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = Rbatis::new();
    rb.init(
        PgDriver {},
        "postgres://postgres:1234@localhost:55435/database",
    )
    .unwrap();
    // orm
    // let mut tx = rb.acquire_begin().await.unwrap();

    // let res = Person::select_all(&mut tx).await.unwrap();
    // println!("Res : \n{:#?}", res);

    // tx.commit().await.unwrap();
    // tx.rollback().await.unwrap();

    // raw sql
    let res: Option<Person> = rb
        .fetch_decode(
            "select * from person where id = ? limit ?",
            vec![to_value!(2), to_value!(1)],
        )
        .await
        .unwrap();
    println!("{:?}", res);
}
