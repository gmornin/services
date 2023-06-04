// use std::time::Duration;
//
// pub const EMAIL_VERIFICATION_DURATION: Duration = Duration::from_secs(3600);

use mongodb::Database;

use crate::{
    functions::{get_client, get_prod_database},
    structs::StorageLimits,
};
use once_cell::sync::OnceCell;
use std::{env, time::Duration};

pub static STORAGE: OnceCell<String> = OnceCell::new();
pub static USERCONTENT: OnceCell<String> = OnceCell::new();
pub static SELF_ADDR: OnceCell<String> = OnceCell::new();
pub static STORAGE_LIMITS: OnceCell<StorageLimits> = OnceCell::new();
pub static EMAIL_VERIFICATION_DURATION: OnceCell<Duration> = OnceCell::new();
pub static DATABASE: OnceCell<Database> = OnceCell::new();

pub async fn init() {
    STORAGE.set(env::var("STORAGE_PATH").unwrap()).unwrap();
    USERCONTENT
        .set(env::var("USERCONTENT_PATH").unwrap())
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
    DATABASE
        .set(get_prod_database(&get_client().await))
        .unwrap();
}

// lazy_static! {
// pub static ref STORAGE: String = env::var("STORAGE_PATH").unwrap();
// pub static ref USERCONTENT: String = env::var("USERCONTENT_PATH").unwrap();
// pub static ref SELF_ADDR: String = env::var("SELF_ADDR").unwrap();
// pub static ref STORAGE_LIMITS: StorageLimits = StorageLimits {
//     _1: env::var("STORAGE_LIMIT_1")
//         .expect("cannot find `STORAGE_LIMIT_1` in env")
//         .parse()
//         .expect("cannot parse STORAGE_LIMIT_1 to u64"),
// };
//     pub static ref EMAIL_VERIFICATION_DURATION: Duration = Duration::from_secs(env::var("EMAIL_VERIFICATION_DURATION").unwrap().parse().unwrap());
//     pub static ref DATABASE: Database = get_prod_database(&get_client());
// }
