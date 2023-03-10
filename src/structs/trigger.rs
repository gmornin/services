use crate::functions::get_triggers;
use crate::structs::GMError;
use crate::traits::{CollectionItem, Triggerable};
use chrono::Utc;
use mongodb::{bson::doc, Database};
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

    pub async fn trigger(id: &str, db: &Database) -> Result<(), Box<dyn Error>> {
        let triggers = get_triggers(db);
        let trigger = triggers
            .find_one_and_delete(doc! {"_id": id}, None)
            .await?
            .ok_or(GMError::TriggerNotFound)?;

        if trigger.is_invalid() {
            return Err(GMError::TriggerNotFound.into());
        }

        trigger.action.trigger(db).await
    }

    pub fn is_invalid(&self) -> bool {
        self.expiry < Utc::now().timestamp() as u64
    }
}

impl CollectionItem for Trigger {
    fn id(&self) -> &str {
        &self.id
    }
}
