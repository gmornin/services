use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_inline_default::serde_inline_default;

use crate::traits::ConfigTrait;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultFromSerde)]
pub struct WhitelistConfig {
    #[serde_inline_default(vec!["127.0.0.1".to_string()])]
    pub create: Vec<String>,
}

impl ConfigTrait for WhitelistConfig {
    const LABEL: &'static str = "whitelist";
}
