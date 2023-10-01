use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_inline_default::serde_inline_default;

use crate::{structs::ItemVisibility, traits::ConfigTrait};

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultFromSerde)]
pub struct DefaultsConfig {
    // #[serde(default = "pfp_default_path_default")]
    // pub pfp_default_path: String,
    #[serde_inline_default(ItemVisibility::Public)]
    pub default_visibility: ItemVisibility,
    #[serde_inline_default(80)]
    pub http_port: u16,
    #[serde_inline_default(443)]
    pub https_port: u16,
}

impl ConfigTrait for DefaultsConfig {
    const LABEL: &'static str = "defaults";
}
