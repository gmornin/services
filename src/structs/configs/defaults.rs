use serde::{Deserialize, Serialize};

use crate::traits::ConfigTrait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultsConfig {
    #[serde(default = "pfp_default_path_default")]
    pub pfp_default_path: String,
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            pfp_default_path: pfp_default_path_default(),
        }
    }
}

impl ConfigTrait for DefaultsConfig {
    const LABEL: &'static str = "defaults";
}

fn pfp_default_path_default() -> String {
    "~/.local/share/gm/default/pfp.svg".to_string()
}
