use std::path::PathBuf;

use mongodb::{bson::Document, options::ClientOptions, Client, Collection, Database};

use crate::{structs::*, *};

pub async fn get_client() -> Client {
    let host = MONGO_HOST.get().unwrap();
    let client_options = ClientOptions::parse(&host)
        .await
        .unwrap_or_else(|_| panic!("cannot resolve mongodb host at `{host}`"));

    Client::with_options(client_options)
        .unwrap_or_else(|_| panic!("cannot connect to mongodb host at `{host}`"))
}

/// Returns handle to database
pub fn get_database(client: &Client, name: &str) -> Database {
    client.database(name)
}

/// Get `accounts` collection
pub fn get_accounts() -> Collection<Account> {
    DATABASE.get().unwrap().collection("accounts")
}

/// Gets `triggers` collection
pub fn get_triggers() -> Collection<Trigger> {
    DATABASE.get().unwrap().collection("triggers")
}

pub fn get_counters() -> Collection<Counter> {
    DATABASE.get().unwrap().collection("counters")
}

pub fn get_counters_doc() -> Collection<Document> {
    DATABASE.get().unwrap().collection("counters")
}

pub fn get_user_dir(id: i64, service: Option<GMServices>) -> PathBuf {
    let mut path = USERCONTENT.get().unwrap().join(id.to_string());
    if let Some(service) = service {
        path.push(service.as_str());
    }
    path
}

pub fn get_usersys_dir(id: i64, service: Option<GMServices>) -> PathBuf {
    get_user_dir(id, service).join(".system")
}
