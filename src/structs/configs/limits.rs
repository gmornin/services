use serde::{Deserialize, Serialize};

use crate::traits::ConfigTrait;

#[derive(Serialize, Deserialize, Clone)]
pub struct LimitsConfig {
    #[serde(default = "verification_timeframe_default")]
    pub verification_timeframe: u64,
    #[serde(default = "verification_timeframe_default")]
    pub verification_cooldown: u64,
    #[serde(default)]
    pub storage_limits: StorageLimitConfigs,
    #[serde(default = "pfp_limit_default")]
    pub pfp_limit: u64,
    #[serde(default)]
    pub jobs: QueueConfigs,
    #[serde(default = "allow_register_default")]
    pub allow_register: bool,
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self {
            verification_timeframe: verification_timeframe_default(),
            verification_cooldown: verification_cooldown_default(),
            storage_limits: StorageLimitConfigs::default(),
            pfp_limit: pfp_limit_default(),
            jobs: QueueConfigs::default(),
            allow_register: allow_register_default(),
        }
    }
}

fn allow_register_default() -> bool {
    true
}

impl ConfigTrait for LimitsConfig {
    const LABEL: &'static str = "limits";
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageLimitConfigs {
    #[serde(default = "base_default")]
    pub base: u64,
}

impl Default for StorageLimitConfigs {
    fn default() -> Self {
        Self {
            base: base_default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueueConfigs {
    #[serde(default = "max_concurrent_default")]
    pub max_concurrent: usize,
    #[serde(default = "queue_limit_default")]
    pub queue_limit: usize,
}

impl Default for QueueConfigs {
    fn default() -> Self {
        Self {
            max_concurrent: max_concurrent_default(),
            queue_limit: queue_limit_default(),
        }
    }
}

const fn verification_timeframe_default() -> u64 {
    3600
}

const fn base_default() -> u64 {
    536870912
}

const fn pfp_limit_default() -> u64 {
    2097152
}

const fn max_concurrent_default() -> usize {
    1
}

const fn queue_limit_default() -> usize {
    9999
}

const fn verification_cooldown_default() -> u64 {
    300
}
