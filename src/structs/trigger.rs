use crate::functions::get_triggers;
use crate::traits::{CollectionItem, Triggerable};
use crate::*;
use chrono::Utc;
use goodmorning_bindings::services::v1::V1Error;
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
        self.action.init(db, &self.id, self.expiry).await
    }

    pub async fn trigger(&self, db: &Database) -> Result<(), Box<dyn Error>> {
        let triggers = get_triggers(db);
        let trigger = triggers
            .find_one_and_delete(doc! {"_id": &self.id}, None)
            .await?
            .ok_or(V1Error::TriggerNotFound)?;

        if trigger.is_invalid() {
            self.delete(&triggers).await?;
            return Err(V1Error::TriggerNotFound.into());
        }

        trigger
            .action
            .trigger(db, &trigger.id, trigger.expiry)
            .await
    }

    pub async fn revoke(&self, db: &Database) -> Result<(), Box<dyn Error>> {
        let triggers = get_triggers(db);
        let trigger = triggers
            .find_one_and_delete(doc! {"_id": &self.id}, None)
            .await?
            .ok_or(V1Error::TriggerNotFound)?;

        if trigger.is_invalid() {
            return Err(V1Error::TriggerNotFound.into());
        }

        trigger.action.revoke(db, &trigger.id, trigger.expiry).await
    }

    pub fn is_invalid(&self) -> bool {
        self.expiry < Utc::now().timestamp() as u64
    }

    pub fn use_url(id: &str) -> String {
        format!(
            "{}/api/triggers/v1/use/{id}",
            SELF_ADDR.get().unwrap().as_str()
        )
    }

    pub fn revoke_url(id: &str) -> String {
        format!(
            "{}/api/triggers/v1/revoke/{id}",
            SELF_ADDR.get().unwrap().as_str()
        )
    }
}

impl CollectionItem<String> for Trigger {
    fn id(&self) -> String {
        self.id.to_string()
    }
}
