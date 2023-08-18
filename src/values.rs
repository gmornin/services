// use std::time::Duration;
//
// pub const EMAIL_VERIFICATION_DURATION: Duration = Duration::from_secs(3600);

use dirs::home_dir;
use lettre::transport::smtp::authentication::Credentials;
use mongodb::{Collection, Database};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde::{Deserialize, Serialize};
use simplelog::*;

use xdg_mime::SharedMimeInfo;

use crate::{
    functions::{get_accounts, get_client, get_counters, get_database, get_triggers, parse_path},
    structs::{
        Account, Counter, CredentialsConfig, DefaultsConfig, ItemVisibility, LimitsConfig,
        StorageConfig, StorageLimitConfigs, Trigger,
    },
    traits::ConfigTrait,
};
use once_cell::sync::OnceCell;
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
    time::Duration,
};

pub static PFP_LIMIT: OnceCell<u64> = OnceCell::new();
pub static QUEUE_LIMIT: OnceCell<usize> = OnceCell::new();
pub static MAX_CONCURRENT: OnceCell<usize> = OnceCell::new();
pub static STORAGE_LIMITS: OnceCell<StorageLimitConfigs> = OnceCell::new();
pub static EMAIL_VERIFICATION_DURATION: OnceCell<Duration> = OnceCell::new();
pub static EMAIL_VERIFICATION_COOLDOWN: OnceCell<u64> = OnceCell::new();
pub static ALLOW_REGISTER: OnceCell<bool> = OnceCell::new();

pub static HASH_SALT: OnceCell<String> = OnceCell::new();
pub static SMTP_USERNAME: OnceCell<String> = OnceCell::new();
pub static SMTP_PASSWORD: OnceCell<String> = OnceCell::new();
pub static SMTP_RELAY: OnceCell<String> = OnceCell::new();
pub static SMTP_FROM: OnceCell<String> = OnceCell::new();
pub static SMTP_CREDS: OnceCell<Credentials> = OnceCell::new();
pub static CERT_CHAIN: OnceCell<PathBuf> = OnceCell::new();
pub static CERT_KEY: OnceCell<PathBuf> = OnceCell::new();

pub static MONGO_HOST: OnceCell<String> = OnceCell::new();
pub static USERCONTENT: OnceCell<PathBuf> = OnceCell::new();
pub static LOGS_PATH: OnceCell<PathBuf> = OnceCell::new();
pub static SELF_ADDR: OnceCell<String> = OnceCell::new();

// pub static PFP_DEFAULT: OnceCell<PathBuf> = OnceCell::new();
pub static VIS_DEFAULT: OnceCell<ItemVisibility> = OnceCell::new();
pub static HTTP_PORT: OnceCell<u16> = OnceCell::new();
pub static HTTPS_PORT: OnceCell<u16> = OnceCell::new();

pub static DATABASE: OnceCell<Database> = OnceCell::new();
pub static ACCOUNTS: OnceCell<Collection<Account>> = OnceCell::new();
pub static TRIGGERS: OnceCell<Collection<Trigger>> = OnceCell::new();
pub static COUNTERS: OnceCell<Collection<Counter>> = OnceCell::new();
pub static MIME_DB: OnceCell<SharedMimeInfo> = OnceCell::new();

pub async fn valinit() {
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
    EMAIL_VERIFICATION_COOLDOWN
        .set(limit_configs.verification_cooldown)
        .unwrap();
    ALLOW_REGISTER.set(limit_configs.allow_register).unwrap();

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
    LOGS_PATH.set(parse_path(storage_config.logs_path)).unwrap();
    USERCONTENT
        .set(parse_path(storage_config.usercontent_path))
        .unwrap();
    SELF_ADDR.set(storage_config.self_addr).unwrap();

    let mongo_client = get_client().await;
    let defaults_config = *DefaultsConfig::load().unwrap();
    // PFP_DEFAULT
    //     .set(parse_path(defaults_config.pfp_default_path))
    //     .unwrap();
    VIS_DEFAULT.set(defaults_config.default_visibility).unwrap();
    HTTP_PORT.set(defaults_config.http_port).unwrap();
    HTTPS_PORT.set(defaults_config.https_port).unwrap();

    DATABASE
        .set(get_database(&mongo_client, &storage_config.db_name))
        .unwrap();
    ACCOUNTS.set(get_accounts()).unwrap();
    TRIGGERS.set(get_triggers()).unwrap();
    COUNTERS.set(get_counters()).unwrap();
    MIME_DB.set(SharedMimeInfo::new()).ok().unwrap();
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum LevelFilterSerde {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LevelFilterSerde> for LevelFilter {
    fn from(val: LevelFilterSerde) -> Self {
        match val {
            LevelFilterSerde::Off => LevelFilter::Off,
            LevelFilterSerde::Warn => LevelFilter::Warn,
            LevelFilterSerde::Info => LevelFilter::Info,
            LevelFilterSerde::Error => LevelFilter::Error,
            LevelFilterSerde::Debug => LevelFilter::Debug,
            LevelFilterSerde::Trace => LevelFilter::Trace,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogOptions {
    pub loglabel: String,
    pub termlogging: bool,
    pub writelogging: bool,
    pub term_log_level: LevelFilterSerde,
    pub write_log_level: LevelFilterSerde,
}

pub async fn init() {
    sudo::escalate_if_needed().unwrap();

    valinit().await;
}

pub fn logs_init(options: &LogOptions) {
    let mut loggers: Vec<Box<dyn SharedLogger + 'static>> = Vec::new();
    let path = LOGS_PATH.get().unwrap();
    if options.termlogging && !path.exists() {
        fs::create_dir_all(path).unwrap();
        loggers.push(TermLogger::new(
            options.term_log_level.into(),
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ));
    }

    if options.writelogging {
        loggers.push(WriteLogger::new(
            options.write_log_level.into(),
            Config::default(),
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(LOGS_PATH.get().unwrap().join(format!(
                    "{}-{}.log",
                    options.loglabel,
                    chrono::Utc::now()
                )))
                .unwrap(),
        ))
    }

    CombinedLogger::init(loggers).unwrap();
}

pub fn load_rustls_config(chain: &PathBuf, key: &PathBuf) -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(chain).unwrap());
    let key_file = &mut BufReader::new(File::open(key).unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
