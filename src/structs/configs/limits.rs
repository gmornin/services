use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_inline_default::serde_inline_default;

use crate::traits::ConfigTrait;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone, DefaultFromSerde)]
pub struct LimitsConfig {
    #[serde_inline_default(3600)]
    pub verification_timeframe: u64,
    #[serde_inline_default(3600)]
    pub verification_cooldown: u64,
    #[serde(default)]
    pub storage_limits: StorageLimitConfigs,
    #[serde_inline_default(2097152)]
    pub pfp_limit: u64,
    #[serde(default)]
    pub jobs: QueueConfigs,
    #[serde_inline_default(true)]
    pub allow_register: bool,
    #[serde_inline_default(true)]
    pub verification: bool,
    #[serde_inline_default(43200)]
    pub storage_size_recheck: u64,
    #[serde(default)]
    pub length_limits: LengthLimits,
}

impl ConfigTrait for LimitsConfig {
    const LABEL: &'static str = "limits";
}

// #[serde_inline_default]
// #[derive(Serialize, Deserialize, Clone, Debug, DefaultFromSerde)]
// pub struct StorageLimitConfigs {
//     #[serde_inline_default(536870912)]
//     pub base: u64,
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageLimitConfigs(pub HashMap<String, u64>);

impl Default for StorageLimitConfigs {
    fn default() -> Self {
        Self(HashMap::from([("base".to_string(), 536870912)]))
    }
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone, DefaultFromSerde)]
pub struct QueueConfigs {
    #[serde_inline_default(1)]
    pub max_concurrent: usize,
    #[serde_inline_default(5)]
    pub queue_limit: usize,
    #[serde_inline_default(HashMap::new())]
    pub presets: HashMap<String, QueueConfig>,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone, DefaultFromSerde)]
pub struct QueueConfig {
    #[serde_inline_default(1)]
    pub max_concurrent: usize,
    #[serde_inline_default(9999)]
    pub queue_limit: usize,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone, DefaultFromSerde)]
pub struct LengthLimits {
    #[serde_inline_default(128)]
    pub status_limit: usize,
    #[serde_inline_default(3)]
    pub username_min: usize,
    #[serde_inline_default(32)]
    pub username_max: usize,
}
