use crate::traits::{CollectionItem, Peekable, Triggerable};
use crate::*;
use async_trait::async_trait;
use chrono::Utc;
use goodmorning_bindings::services::v1::V1Error;
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

/// Action that will only be performed when the trigger url is visited
#[derive(Serialize, Deserialize, Clone, Debug)]
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

    pub async fn init(&self, _db: &Database) -> Result<(), Box<dyn Error>> {
        self.action.init(&self.id, self.expiry).await
    }

    pub async fn trigger(&self, _db: &Database) -> Result<(), Box<dyn Error>> {
        let triggers = TRIGGERS.get().unwrap();
        let trigger = triggers
            .find_one_and_delete(doc! {"_id": &self.id})
            .await?
            .ok_or(V1Error::TriggerNotFound)?;

        if trigger.is_invalid() {
            self.delete(triggers).await?;
            return Err(V1Error::TriggerNotFound.into());
        }

        trigger.action.trigger(&trigger.id, trigger.expiry).await
    }

    pub async fn revoke(&self, db: &Database) -> Result<(), Box<dyn Error>> {
        let triggers = TRIGGERS.get().unwrap();
        let trigger = triggers
            .find_one_and_delete(doc! {"_id": &self.id})
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

    pub fn url(id: &str) -> String {
        format!("{}/trigger/{id}", SELF_ADDR.get().unwrap().as_str())
    }

    pub fn peek(&self) -> Option<Box<dyn Peekable>> {
        self.action.peek(&self.id, self.expiry)
    }
}

#[async_trait]
impl CollectionItem<String> for Trigger {
    fn id(&self) -> String {
        self.id.to_string()
    }

    async fn find_by_id(
        id: String,
        collection: &Collection<Self>,
    ) -> Result<Option<Self>, mongodb::error::Error> {
        let trigger = collection.find_one(doc! {"_id": id}).await?;

        if let Some(trigger) = &trigger
            && trigger.is_invalid()
        {
            let _ = trigger.revoke(DATABASE.get().unwrap()).await;
            let _ = trigger.delete(collection).await;
            return Ok(None);
        }

        Ok(trigger)
    }
}
