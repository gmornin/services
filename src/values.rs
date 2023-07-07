// use std::time::Duration;
//
// pub const EMAIL_VERIFICATION_DURATION: Duration = Duration::from_secs(3600);

use mongodb::{Collection, Database};

use xdg_mime::SharedMimeInfo;

use crate::{
    functions::{get_accounts, get_client, get_counters, get_database, get_triggers},
    structs::{Account, Counter, StorageLimits, Trigger},
};
use once_cell::sync::OnceCell;
use std::{env, path::PathBuf, time::Duration};

pub static STORAGE: OnceCell<PathBuf> = OnceCell::new();
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
pub static ACCOUNTS: OnceCell<Collection<Account>> = OnceCell::new();
pub static TRIGGERS: OnceCell<Collection<Trigger>> = OnceCell::new();
pub static COUNTERS: OnceCell<Collection<Counter>> = OnceCell::new();
pub static QUEUE_LIMIT: OnceCell<usize> = OnceCell::new();
pub static MAX_CONCURRENT: OnceCell<usize> = OnceCell::new();

pub async fn init() {
    MIME_DB.set(SharedMimeInfo::new()).ok().unwrap();
    MONGO_HOST.set(env::var("MONGO_HOST").unwrap()).unwrap();
    PFP_LIMIT
        .set(env::var("PFP_LIMIT").unwrap().parse::<u64>().unwrap())
        .unwrap();
    DB_NAME.set(env::var("DB_NAME").unwrap()).unwrap();
    QUEUE_LIMIT
        .set(env::var("QUEUE_LIMIT").unwrap().parse().unwrap())
        .unwrap();
    MAX_CONCURRENT
        .set(env::var("MAX_CONCURRENT").unwrap().parse().unwrap())
        .unwrap();
    STORAGE
        .set(parse_path(env::var("STORAGE_PATH").unwrap()))
        .unwrap();
    USERCONTENT
        .set(parse_path(env::var("USERCONTENT_PATH").unwrap()))
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
    ACCOUNTS.set(get_accounts(DATABASE.get().unwrap())).unwrap();
    TRIGGERS.set(get_triggers(DATABASE.get().unwrap())).unwrap();
    COUNTERS.set(get_counters(DATABASE.get().unwrap())).unwrap();
}

fn parse_path(mut s: String) -> PathBuf {
    if s.starts_with('~') {
        s = s.replacen(
            '~',
            dirs::home_dir().unwrap().as_os_str().to_str().unwrap(),
            1,
        );
    }

    PathBuf::from(s)
}
