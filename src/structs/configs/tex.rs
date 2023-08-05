use serde::{Deserialize, Serialize};

use crate::traits::ConfigTrait;

#[derive(Serialize, Deserialize, Clone)]
pub struct TexConfig {
    #[serde(default = "db_name_default")]
    pub db_name: String,
}

impl Default for TexConfig {
    fn default() -> Self {
        Self {
            db_name: db_name_default(),
        }
    }
}

impl ConfigTrait for TexConfig {
    const LABEL: &'static str = "tex";
}

fn db_name_default() -> String {
    "gmtex".to_string()
}
