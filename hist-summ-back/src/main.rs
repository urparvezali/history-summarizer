pub mod entity;

#[allow(unused)]
use entity::prelude::*;
use sea_orm::{Database, EntityTrait};

#[tokio::main]
async fn main() {
    let db = Database::connect("mysql://parvez:mypass@localhost/histsumm")
        .await
        .unwrap();
    let res = Users::find().all(&db).await.unwrap();
    for user in res {
        println!("{:?}", user);
    }
}
