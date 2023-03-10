use crate::traits::{CollectionItem, Triggerable};
use chrono::Utc;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

/// Action that will only be performed when the trigger url is visited
#[derive(Serialize, Deserialize, Clone)]
pub struct Trigger {
    /// Id of the trigger, for example when visited `localhost/trigger/{id}`, that action will be
    /// triggered
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: String,
    pub expiry: u64,

    pub action: Box<dyn Triggerable>,
}

/// The action to be performed
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TriggeredAction {
    #[serde(rename(serialize = "emailVerification", deserialize = "emailVerification"))]
    EmailVerification {
        email: String,
        /// ID of the user to verify
        id: String,
    },
}

impl Trigger {
    /// Create new `Self` from `TriggeredAction`
    pub fn new_from_action(action: Box<dyn Triggerable>, duration: &Duration) -> Self {
        Self {
            id: hex::encode(fastrand::u128(..).to_be_bytes()),
            expiry: Utc::now().timestamp() as u64 + duration.as_secs(),

            action,
        }
    }

    pub async fn init(&self, db: &Database) -> Result<(), Box<dyn Error>> {
        self.action.init(db).await
    }

    pub async fn trigger(&self, db: &Database) -> Result<(), Box<dyn Error>> {
        let triggers
    }

    pub fn is_valid(&self) -> bool {
        self.expiry > Utc::now().timestamp() as u64
    }
}

impl CollectionItem for Trigger {
    fn id(&self) -> &str {
        &self.id
    }
}
