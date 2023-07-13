use crate::functions as f;
use dotenv::dotenv;

#[tokio::test]
async fn get_client() {
    f::get_client().await;
}

#[tokio::test]
async fn get_database() {
    f::get_dev_database(&f::get_client().await);
}

#[tokio::test]
async fn get_accounts() {
    f::get_accounts(&f::get_dev_database(&f::get_client().await));
}
