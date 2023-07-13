// use std::time::Duration;
//
// pub const EMAIL_VERIFICATION_DURATION: Duration = Duration::from_secs(3600);

use dirs::home_dir;
use lettre::transport::smtp::authentication::Credentials;
use mongodb::{Collection, Database};

use xdg_mime::SharedMimeInfo;

use crate::{
    functions::{get_accounts, get_client, get_counters, get_database, get_triggers},
    structs::{
        Account, Counter, CredentialsConfig, DefaultsConfig, LimitsConfig, StorageConfig,
        StorageLimitConfigs, Trigger,
    },
    traits::ConfigTrait,
};
use once_cell::sync::OnceCell;
use std::{fs, path::PathBuf, time::Duration};

pub static PFP_LIMIT: OnceCell<u64> = OnceCell::new();
pub static QUEUE_LIMIT: OnceCell<usize> = OnceCell::new();
pub static MAX_CONCURRENT: OnceCell<usize> = OnceCell::new();
pub static STORAGE_LIMITS: OnceCell<StorageLimitConfigs> = OnceCell::new();
pub static EMAIL_VERIFICATION_DURATION: OnceCell<Duration> = OnceCell::new();

pub static HASH_SALT: OnceCell<String> = OnceCell::new();
pub static SMTP_USERNAME: OnceCell<String> = OnceCell::new();
pub static SMTP_PASSWORD: OnceCell<String> = OnceCell::new();
pub static SMTP_RELAY: OnceCell<String> = OnceCell::new();
pub static SMTP_FROM: OnceCell<String> = OnceCell::new();
pub static SMTP_CREDS: OnceCell<Credentials> = OnceCell::new();
pub static CERT_CHAIN: OnceCell<PathBuf> = OnceCell::new();
pub static CERT_KEY: OnceCell<PathBuf> = OnceCell::new();

pub static MONGO_HOST: OnceCell<String> = OnceCell::new();
pub static DB_NAME: OnceCell<String> = OnceCell::new();
pub static STORAGE: OnceCell<PathBuf> = OnceCell::new();
pub static USERCONTENT: OnceCell<PathBuf> = OnceCell::new();
pub static SELF_ADDR: OnceCell<String> = OnceCell::new();

pub static PFP_DEFAULT: OnceCell<PathBuf> = OnceCell::new();

pub static DATABASE: OnceCell<Database> = OnceCell::new();
pub static ACCOUNTS: OnceCell<Collection<Account>> = OnceCell::new();
pub static TRIGGERS: OnceCell<Collection<Trigger>> = OnceCell::new();
pub static COUNTERS: OnceCell<Collection<Counter>> = OnceCell::new();
pub static MIME_DB: OnceCell<SharedMimeInfo> = OnceCell::new();

pub async fn init() {
    let configs = home_dir().unwrap().join(".config/gm/");

    if !configs.exists() {
        fs::create_dir_all(configs).unwrap();
    }

    let limit_configs = *LimitsConfig::load().unwrap();
    PFP_LIMIT.set(limit_configs.pfp_limit).unwrap();
    QUEUE_LIMIT.set(limit_configs.jobs.queue_limit).unwrap();
    MAX_CONCURRENT
        .set(limit_configs.jobs.max_concurrent)
        .unwrap();
    STORAGE_LIMITS.set(limit_configs.storage_limits).unwrap();
    EMAIL_VERIFICATION_DURATION
        .set(Duration::from_secs(limit_configs.verification_timeframe))
        .unwrap();

    let cert_config = *CredentialsConfig::load().unwrap();
    HASH_SALT.set(cert_config.hash_salt).unwrap();
    SMTP_RELAY.set(cert_config.smtp.relay).unwrap();
    SMTP_FROM.set(cert_config.smtp.from).unwrap();
    SMTP_CREDS
        .set(Credentials::new(
            cert_config.smtp.username,
            cert_config.smtp.password,
        ))
        .unwrap();
    CERT_KEY.set(parse_path(cert_config.ssl_paths.key)).unwrap();
    CERT_CHAIN
        .set(parse_path(cert_config.ssl_paths.chain))
        .unwrap();

    let storage_config = *StorageConfig::load().unwrap();
    MONGO_HOST.set(storage_config.mongo_host).unwrap();
    DB_NAME.set(storage_config.db_name).unwrap();
    STORAGE
        .set(parse_path(storage_config.storage_path))
        .unwrap();
    USERCONTENT
        .set(parse_path(storage_config.usercontent_path))
        .unwrap();
    SELF_ADDR.set(storage_config.self_addr).unwrap();

    let defaults_config = *DefaultsConfig::load().unwrap();
    PFP_DEFAULT
        .set(parse_path(defaults_config.pfp_default_path))
        .unwrap();

    DATABASE.set(get_database(&get_client().await)).unwrap();
    ACCOUNTS.set(get_accounts(DATABASE.get().unwrap())).unwrap();
    TRIGGERS.set(get_triggers(DATABASE.get().unwrap())).unwrap();
    COUNTERS.set(get_counters(DATABASE.get().unwrap())).unwrap();
    MIME_DB.set(SharedMimeInfo::new()).ok().unwrap();
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
