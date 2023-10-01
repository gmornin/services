use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_inline_default::serde_inline_default;

use crate::traits::ConfigTrait;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultFromSerde)]
pub struct CredentialsConfig {
    #[serde_inline_default(format!("change me -> {}",hex::encode(fastrand::u128(..).to_be_bytes())))]
    pub hash_salt: String,
    #[serde(default)]
    pub smtp: SmtpConfig,
    #[serde(default)]
    pub ssl_paths: SslConfig,
}

impl ConfigTrait for CredentialsConfig {
    const LABEL: &'static str = "credentials";
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultFromSerde)]
pub struct SmtpConfig {
    #[serde_inline_default("your email username (e.g. username@gmail.com)".to_string())]
    pub username: String,
    #[serde_inline_default("your smtp password, usually looks gibberish".to_string())]
    pub password: String,
    #[serde_inline_default("such as smtp.gmail.com".to_string())]
    pub relay: String,
    #[serde_inline_default("something like `GoodMorningTex<username@gmail.com>`".to_string())]
    pub from: String,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultFromSerde)]
pub struct SslConfig {
    #[serde_inline_default("change me path to chain file /etc/letsencrypt/live/yourdomain.com/fullchain.pem".to_string())]
    pub chain: String,
    #[serde_inline_default("change me path to private key /etc/letsencrypt/live/yourdomain.com/privkey.pem".to_string())]
    pub key: String,
}
