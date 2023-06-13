use mongodb::{bson::Document, options::ClientOptions, Client, Collection, Database};

use crate::{
    structs::{Account, Counter, Trigger},
    DB_NAME, MONGO_HOST,
};

pub async fn get_client() -> Client {
    let host = MONGO_HOST.get().unwrap();
    let client_options = ClientOptions::parse(&host)
        .await
        .unwrap_or_else(|_| panic!("cannot resolve mongodb host at `{host}`"));

    Client::with_options(client_options)
        .unwrap_or_else(|_| panic!("cannot connect to mongodb host at `{host}`"))
}

/// Returns handle to database
pub fn get_database(client: &Client) -> Database {
    client.database(DB_NAME.get().unwrap())
}

/// Get `accounts` collection
pub fn get_accounts(db: &Database) -> Collection<Account> {
    db.collection("accounts")
}

/// Gets `triggers` collection
pub fn get_triggers(db: &Database) -> Collection<Trigger> {
    db.collection("triggers")
}

pub fn get_counters(db: &Database) -> Collection<Counter> {
    db.collection("counters")
}

pub fn get_counters_doc(db: &Database) -> Collection<Document> {
    db.collection("counters")
}
