use sea_orm::Database;

#[tokio::main]
async fn main() {
    let _ = Database::connect("mysql://parvez:mypass@localhost/histsumm")
        .await
        .unwrap();
}
