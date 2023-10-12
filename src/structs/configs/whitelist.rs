use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_inline_default::serde_inline_default;

use crate::traits::ConfigTrait;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultFromSerde)]
pub struct WhitelistConfig {
    #[serde_inline_default(vec!["127.0.0.1".to_string()])]
    pub create: Vec<String>,
    #[serde(default)]
    pub file_check: FileCheckWhitelist,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultFromSerde)]
pub struct FileCheckWhitelist {
    #[serde_inline_default(vec!["text".to_string(), "application".to_string()])]
    pub mime_fronttype: Vec<String>,
    #[serde(default)]
    pub mime_subtype: Vec<String>,
    #[serde(default)]
    pub file_exts: Vec<String>,
}

impl ConfigTrait for WhitelistConfig {
    const LABEL: &'static str = "whitelist";
}
