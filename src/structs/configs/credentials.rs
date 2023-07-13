use serde::{Deserialize, Serialize};

use crate::traits::ConfigTrait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CredentialsConfig {
    #[serde(default = "hash_salt_default")]
    pub hash_salt: String,
    #[serde(default)]
    pub smtp: SmtpConfig,
    #[serde(default)]
    pub ssl_paths: SslConfig,
}

impl Default for CredentialsConfig {
    fn default() -> Self {
        Self {
            hash_salt: hash_salt_default(),
            smtp: SmtpConfig::default(),
            ssl_paths: SslConfig::default(),
        }
    }
}

impl ConfigTrait for CredentialsConfig {
    const LABEL: &'static str = "credentials";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmtpConfig {
    #[serde(default = "username_default")]
    pub username: String,
    #[serde(default = "password_default")]
    pub password: String,
    #[serde(default = "relay_default")]
    pub relay: String,
    #[serde(default = "from_default")]
    pub from: String,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            username: username_default(),
            password: password_default(),
            relay: relay_default(),
            from: from_default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SslConfig {
    #[serde(default = "chain_default")]
    pub chain: String,
    #[serde(default = "key_default")]
    pub key: String,
}

impl Default for SslConfig {
    fn default() -> Self {
        Self {
            chain: chain_default(),
            key: key_default(),
        }
    }
}

fn username_default() -> String {
    "your email username (e.g. username@gmail.com)".to_string()
}

fn password_default() -> String {
    "your smtp password, usually looks gibberish".to_string()
}

fn relay_default() -> String {
    "such as smtp.gmail.com".to_string()
}

fn from_default() -> String {
    "something like `GoodMorningTex<username@gmail.com>`".to_string()
}

fn chain_default() -> String {
    "change me path to chain file /etc/letsencrypt/live/yourdomain.com/fullchain.pem".to_string()
}

fn key_default() -> String {
    "change me path to private key /etc/letsencrypt/live/yourdomain.com/privkey.pem".to_string()
}

fn hash_salt_default() -> String {
    format!(
        "change me -> {}",
        hex::encode(fastrand::u128(..).to_be_bytes())
    )
}
