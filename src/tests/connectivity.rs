use crate::functions as f;
use dotenv::dotenv;

#[tokio::test]
async fn get_client() {
    dotenv().ok();
    f::get_client().await;
}

#[tokio::test]
async fn get_database() {
    dotenv().ok();
    f::get_dev_database(&f::get_client().await);
}

#[tokio::test]
async fn get_accounts() {
    dotenv().ok();
    f::get_accounts(&f::get_dev_database(&f::get_client().await));
}
