use serde::{Deserialize, Serialize};

use crate::{structs::ItemVisibility, traits::ConfigTrait};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultsConfig {
    #[serde(default = "pfp_default_path_default")]
    pub pfp_default_path: String,
    #[serde(default = "default_visibility_default")]
    pub default_visibility: ItemVisibility,
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            pfp_default_path: pfp_default_path_default(),
            default_visibility: default_visibility_default(),
        }
    }
}

impl ConfigTrait for DefaultsConfig {
    const LABEL: &'static str = "defaults";
}

fn pfp_default_path_default() -> String {
    "~/.local/share/gm/default/pfp.svg".to_string()
}

fn default_visibility_default() -> ItemVisibility {
    ItemVisibility::Public
}
