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
    #[serde_inline_default(4)]
    pub token_length: u32,
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
