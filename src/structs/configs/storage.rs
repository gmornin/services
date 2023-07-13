use serde::{Deserialize, Serialize};

use crate::traits::ConfigTrait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StorageConfig {
    #[serde(default = "self_addr_default")]
    pub self_addr: String,
    #[serde(default = "mongodb_host_default")]
    pub mongo_host: String,
    #[serde(default = "db_name_default")]
    pub db_name: String,
    #[serde(default = "storage_path_default")]
    pub storage_path: String,
    #[serde(default = "usercontent_path_default")]
    pub usercontent_path: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            self_addr: self_addr_default(),
            mongo_host: mongodb_host_default(),
            db_name: db_name_default(),
            storage_path: storage_path_default(),
            usercontent_path: usercontent_path_default(),
        }
    }
}

impl ConfigTrait for StorageConfig {
    const LABEL: &'static str = "storage";
}

fn self_addr_default() -> String {
    "such as https://this.site".to_string()
}

fn mongodb_host_default() -> String {
    "mongodb://localhost:27017".to_string()
}

fn db_name_default() -> String {
    "goodmorning-prod".to_string()
}

fn storage_path_default() -> String {
    "~/.local/share/gm".to_string()
}

fn usercontent_path_default() -> String {
    "~/.local/share/gm/usercontent".to_string()
}
