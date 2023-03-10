use mongodb::{options::ClientOptions, Client, Collection, Database};
use std::env;

use crate::structs::{Account, Trigger};

/// Gets `Client` with host at `MONGO_HOST`
pub async fn get_client() -> Client {
    let host = env::var("MONGO_HOST").expect("`MONGO_HOST` not present in env");
    let client_options = ClientOptions::parse(&host)
        .await
        .unwrap_or_else(|_| panic!("cannot resolve mongodb host at `{host}`"));

    Client::with_options(client_options)
        .unwrap_or_else(|_| panic!("cannot connect to mongodb host at `{host}`"))
}

/// Returns handle to production database
pub fn get_prod_database(client: &Client) -> Database {
    client.database("goodmorning-prod")
}

/// Returns handle to development database
pub fn get_dev_database(client: &Client) -> Database {
    client.database("goodmorning-dev")
}

/// Get `accounts` collection
pub fn get_accounts(db: &Database) -> Collection<Account> {
    db.collection("accounts")
}

pub fn get_triggers(db: &Database) -> Collection<Trigger> {
    db.collection("triggers")
}
