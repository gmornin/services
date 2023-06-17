// use std::time::Duration;
//
// pub const EMAIL_VERIFICATION_DURATION: Duration = Duration::from_secs(3600);

use mongodb::Database;
use xdg_mime::SharedMimeInfo;

use crate::{
    functions::{get_client, get_database},
    structs::StorageLimits,
};
use once_cell::sync::OnceCell;
use std::{env, path::PathBuf, time::Duration};

pub static STORAGE: OnceCell<String> = OnceCell::new();
pub static USERCONTENT: OnceCell<PathBuf> = OnceCell::new();
pub static SELF_ADDR: OnceCell<String> = OnceCell::new();
pub static STORAGE_LIMITS: OnceCell<StorageLimits> = OnceCell::new();
pub static EMAIL_VERIFICATION_DURATION: OnceCell<Duration> = OnceCell::new();
pub static DATABASE: OnceCell<Database> = OnceCell::new();
pub static MONGO_HOST: OnceCell<String> = OnceCell::new();
pub static DB_NAME: OnceCell<String> = OnceCell::new();
pub static PFP_LIMIT: OnceCell<u64> = OnceCell::new();
pub static MIME_DB: OnceCell<SharedMimeInfo> = OnceCell::new();
pub static PFP_DEFAULT: OnceCell<PathBuf> = OnceCell::new();

pub async fn init() {
    MIME_DB.set(SharedMimeInfo::new()).ok().unwrap();
    MONGO_HOST.set(env::var("MONGO_HOST").unwrap()).unwrap();
    PFP_LIMIT
        .set(env::var("PFP_LIMIT").unwrap().parse::<u64>().unwrap())
        .unwrap();
    DB_NAME.set(env::var("DB_NAME").unwrap()).unwrap();
    STORAGE.set(env::var("STORAGE_PATH").unwrap()).unwrap();
    USERCONTENT
        .set(PathBuf::from(env::var("USERCONTENT_PATH").unwrap()))
        .unwrap();
    SELF_ADDR.set(env::var("SELF_ADDR").unwrap()).unwrap();
    STORAGE_LIMITS
        .set(StorageLimits {
            _1: env::var("STORAGE_LIMIT_1")
                .expect("cannot find `STORAGE_LIMIT_1` in env")
                .parse()
                .expect("cannot parse STORAGE_LIMIT_1 to u64"),
        })
        .unwrap();
    EMAIL_VERIFICATION_DURATION
        .set(Duration::from_secs(
            env::var("VERIFICATION_TIMEFRAME").unwrap().parse().unwrap(),
        ))
        .unwrap();
    DATABASE.set(get_database(&get_client().await)).unwrap();
    PFP_DEFAULT
        .set(PathBuf::from(env::var("PFP_DEFAULT").unwrap()))
        .unwrap();
}
