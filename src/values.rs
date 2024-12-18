// use std::time::Duration;
//
// pub const EMAIL_VERIFICATION_DURATION: Duration = Duration::from_secs(3600);

use dirs::home_dir;
use lettre::transport::smtp::authentication::Credentials;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use simplelog::*;

use xdg_mime::SharedMimeInfo;

use crate::{
    functions::{get_accounts, get_client, get_counters, get_database, get_triggers, parse_path},
    structs::{
        Account, Counter, CredentialsConfig, DefaultsConfig, EmailLists, FileCheckType,
        ItemVisibility, LimitsConfig, QueueConfig, StorageConfig, StorageLimitConfigs, Trigger,
        WhitelistConfig,
    },
    traits::ConfigTrait,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
    sync::OnceLock,
    time::Duration,
};

pub static PFP_LIMIT: OnceLock<u64> = OnceLock::new();
pub static QUEUE_LIMIT: OnceLock<usize> = OnceLock::new();
pub static QUEUE_PRESETS: OnceLock<HashMap<String, QueueConfig>> = OnceLock::new();
pub static MAX_CONCURRENT: OnceLock<usize> = OnceLock::new();
pub static STORAGE_LIMITS: OnceLock<StorageLimitConfigs> = OnceLock::new();
pub static EMAIL_VERIFICATION_DURATION: OnceLock<Duration> = OnceLock::new();
pub static INVITE_DURATION: OnceLock<Duration> = OnceLock::new();
pub static EMAIL_VERIFICATION_COOLDOWN: OnceLock<u64> = OnceLock::new();
pub static ALLOW_REGISTER: OnceLock<bool> = OnceLock::new();
pub static ALLOW_INVITE: OnceLock<bool> = OnceLock::new();
pub static VERIFICATION: OnceLock<bool> = OnceLock::new();
pub static STATUS_LENGTH_LIMIT: OnceLock<usize> = OnceLock::new();
pub static STORAGE_SIZE_RECHECK: OnceLock<u64> = OnceLock::new();
pub static USERNAME_MIN: OnceLock<usize> = OnceLock::new();
pub static USERNAME_MAX: OnceLock<usize> = OnceLock::new();

pub static HASH_SALT: OnceLock<String> = OnceLock::new();
pub static SMTP_USERNAME: OnceLock<String> = OnceLock::new();
pub static SMTP_PASSWORD: OnceLock<String> = OnceLock::new();
pub static SMTP_RELAY: OnceLock<String> = OnceLock::new();
pub static SMTP_FROM: OnceLock<String> = OnceLock::new();
pub static SMTP_CREDS: OnceLock<Credentials> = OnceLock::new();
pub static TOKEN_LENGTH: OnceLock<u32> = OnceLock::new();

pub static MONGO_HOST: OnceLock<String> = OnceLock::new();
pub static USERCONTENT: OnceLock<PathBuf> = OnceLock::new();
pub static LOGS_PATH: OnceLock<PathBuf> = OnceLock::new();
pub static SELF_ADDR: OnceLock<String> = OnceLock::new();
pub static SERVICES_STATIC: OnceLock<PathBuf> = OnceLock::new();

pub static FILE_CHECK: OnceLock<FileCheckType> = OnceLock::new();
pub static FILE_CHECK_MIMETYPES: OnceLock<HashSet<String>> = OnceLock::new();
pub static FILE_CHECK_SUBTYPES: OnceLock<HashSet<String>> = OnceLock::new();
pub static FILE_CHECK_EXT: OnceLock<HashSet<String>> = OnceLock::new();

// pub static PFP_DEFAULT: OnceLock<PathBuf> = OnceLock::new();
pub static VIS_DEFAULT: OnceLock<ItemVisibility> = OnceLock::new();
pub static PORT: OnceLock<u16> = OnceLock::new();
pub static FORWARDED: OnceLock<bool> = OnceLock::new();

pub static CREATE_WHITELIST: OnceLock<Vec<String>> = OnceLock::new();
pub static INVITE_WHITELIST: OnceLock<HashSet<i64>> = OnceLock::new();
pub static EMAIL_WHITELIST: OnceLock<EmailLists> = OnceLock::new();

pub static DATABASE: OnceLock<Database> = OnceLock::new();
pub static ACCOUNTS: OnceLock<Collection<Account>> = OnceLock::new();
pub static TRIGGERS: OnceLock<Collection<Trigger>> = OnceLock::new();
pub static COUNTERS: OnceLock<Collection<Counter>> = OnceLock::new();
pub static MIME_DB: OnceLock<SharedMimeInfo> = OnceLock::new();

pub async fn valinit() {
    let configs = home_dir().unwrap().join(".config/gm/");

    if !configs.exists() {
        fs::create_dir_all(configs).unwrap();
    }

    let limit_configs = *LimitsConfig::load().unwrap();
    PFP_LIMIT.set(limit_configs.pfp_limit).unwrap();
    QUEUE_LIMIT.set(limit_configs.jobs.queue_limit).unwrap();
    let _ = QUEUE_PRESETS.set(limit_configs.jobs.presets);
    MAX_CONCURRENT
        .set(limit_configs.jobs.max_concurrent)
        .unwrap();
    STORAGE_LIMITS.set(limit_configs.storage_limits).unwrap();
    INVITE_DURATION
        .set(Duration::from_secs(limit_configs.invite_timeframe))
        .unwrap();
    EMAIL_VERIFICATION_DURATION
        .set(Duration::from_secs(limit_configs.verification_timeframe))
        .unwrap();
    EMAIL_VERIFICATION_COOLDOWN
        .set(limit_configs.verification_cooldown)
        .unwrap();
    ALLOW_REGISTER.set(limit_configs.allow_register).unwrap();
    ALLOW_INVITE.set(limit_configs.allow_invites).unwrap();
    VERIFICATION.set(limit_configs.verification).unwrap();
    STATUS_LENGTH_LIMIT
        .set(limit_configs.length_limits.status_limit)
        .unwrap();
    STORAGE_SIZE_RECHECK
        .set(limit_configs.storage_size_recheck)
        .unwrap();
    USERNAME_MIN
        .set(limit_configs.length_limits.username_min)
        .unwrap();
    USERNAME_MAX
        .set(limit_configs.length_limits.username_max)
        .unwrap();

    let creds_config = *CredentialsConfig::load().unwrap();
    HASH_SALT.set(creds_config.hash_salt).unwrap();
    SMTP_RELAY.set(creds_config.smtp.relay).unwrap();
    SMTP_FROM.set(creds_config.smtp.from).unwrap();
    SMTP_CREDS
        .set(Credentials::new(
            creds_config.smtp.username,
            creds_config.smtp.password,
        ))
        .unwrap();
    TOKEN_LENGTH.set(creds_config.token_length).unwrap();

    let storage_config = *StorageConfig::load().unwrap();
    MONGO_HOST.set(storage_config.mongo_host).unwrap();
    LOGS_PATH.set(parse_path(storage_config.logs_path)).unwrap();
    USERCONTENT
        .set(parse_path(storage_config.usercontent_path))
        .unwrap();
    SELF_ADDR.set(storage_config.self_addr).unwrap();
    FILE_CHECK.set(storage_config.file_check).unwrap();
    SERVICES_STATIC
        .set(parse_path(storage_config.static_path))
        .unwrap();

    let mut whitelist_config = *WhitelistConfig::load().unwrap();
    whitelist_config.create.sort();
    CREATE_WHITELIST.set(whitelist_config.create).unwrap();
    INVITE_WHITELIST.set(whitelist_config.invite).unwrap();
    FILE_CHECK_MIMETYPES
        .set(HashSet::from_iter(
            whitelist_config.file_check.mime_fronttype,
        ))
        .unwrap();
    FILE_CHECK_SUBTYPES
        .set(HashSet::from_iter(whitelist_config.file_check.mime_subtype))
        .unwrap();
    FILE_CHECK_EXT
        .set(HashSet::from_iter(whitelist_config.file_check.file_exts))
        .unwrap();
    EMAIL_WHITELIST.set(whitelist_config.emails.load()).unwrap();

    let mongo_client = get_client().await;
    let defaults_config = *DefaultsConfig::load().unwrap();
    // PFP_DEFAULT
    //     .set(parse_path(defaults_config.pfp_default_path))
    //     .unwrap();
    VIS_DEFAULT.set(defaults_config.default_visibility).unwrap();
    PORT.set(defaults_config.http_port).unwrap();
    FORWARDED.set(defaults_config.forwarded).unwrap();

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
    valinit().await;
}

pub fn logs_init(options: &LogOptions) {
    let mut loggers: Vec<Box<dyn SharedLogger + 'static>> = Vec::new();
    let path = LOGS_PATH.get().unwrap();
    if options.termlogging {
        loggers.push(TermLogger::new(
            options.term_log_level.into(),
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ));
    }

    if options.writelogging {
        if !path.exists() {
            fs::create_dir_all(path).unwrap();
        }
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
