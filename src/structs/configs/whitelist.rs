use std::{collections::HashSet, fs};

use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_inline_default::serde_inline_default;

use crate::{functions::parse_path, traits::ConfigTrait};

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultFromSerde)]
pub struct WhitelistConfig {
    #[serde_inline_default(vec!["127.0.0.1".to_string()])]
    pub create: Vec<String>,
    #[serde_inline_default(HashSet::new())]
    pub invite: HashSet<i64>,
    #[serde(default)]
    pub file_check: FileCheckWhitelist,
    #[serde(default)]
    pub emails: EmailLists,
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

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultFromSerde)]
pub struct EmailLists {
    #[serde(default)]
    pub mode: EmailMode,
    #[serde_inline_default("path/to/list/of/domains.txt".to_string())]
    pub domains_list: String,
    #[serde(default)]
    #[serde(skip_serializing)]
    inner_set: HashSet<String>,
}

impl EmailLists {
    pub fn load(mut self) -> Self {
        if self.mode != EmailMode::None {
            for dom in fs::read_to_string(parse_path(self.domains_list.clone()))
                .unwrap()
                .lines()
            {
                self.inner_set.insert(dom.to_string());
            }
        }

        self
    }

    pub fn allow(&self, email: &str) -> bool {
        match self.mode {
            EmailMode::None => true,
            EmailMode::Whitelist => self.inner_set.contains(email),
            EmailMode::Blacklist => !self.inner_set.contains(email),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EmailMode {
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist,
    #[serde(rename = "none")]
    None,
}

impl Default for EmailMode {
    fn default() -> Self {
        Self::None
    }
}
