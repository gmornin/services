use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_inline_default::serde_inline_default;

use crate::traits::ConfigTrait;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultFromSerde)]
pub struct StorageConfig {
    #[serde_inline_default("such as https://this.site".to_string())]
    pub self_addr: String,
    #[serde_inline_default("mongodb://localhost:27017".to_string())]
    pub mongo_host: String,
    #[serde_inline_default("goodmorning-prod".to_string())]
    pub db_name: String,
    #[serde_inline_default("~/.local/share/gm/usercontent".to_string())]
    pub usercontent_path: String,
    #[serde_inline_default("~/.local/share/gm/logs".to_string())]
    pub logs_path: String,
}

impl ConfigTrait for StorageConfig {
    const LABEL: &'static str = "storage";
}
