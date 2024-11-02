use crate::{
    structs::{ServiceTriggerWrapper, Trigger},
    traits::{CollectionItem, Peekable, Triggerable},
    *,
};
use async_trait::async_trait;

use bindings::structs::{ServicesTriggerTypes, ServicesTriggers};
use goodmorning_bindings::services::v1::V1Error;
use lettre::{
    message::{header::ContentType, MessageBuilder},
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmailVerification {
    pub email: String,
    pub username: String,
    pub id: i64,
}

#[typetag::serde]
#[async_trait]
impl Triggerable for EmailVerification {
    async fn init(&self, id: &str, _expiry: u64) -> Result<(), Box<dyn Error>> {
        let message = MessageBuilder::new()
            .from(SMTP_FROM.get().unwrap().parse()?)
            .to(format!("{}<{}>", self.username, self.email).parse()?)
            .subject("Verify your GoodMorning account")
            .header(ContentType::TEXT_PLAIN)
            .body(format!(
                "Verification link: {}\n\nAccount details:\n  Username: {}\n  User ID: {}",
                Trigger::url(id),
                self.username,
                self.id,
            ))?;

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(SMTP_RELAY.get().unwrap())?
            .credentials(SMTP_CREDS.get().unwrap().clone())
            .build();

        mailer.send(message).await?;

        Ok(())
    }

    async fn trigger(&self, _id: &str, _expiry: u64) -> Result<(), Box<dyn Error>> {
        let accounts = ACCOUNTS.get().unwrap();
        let mut account = accounts
            .find_one(doc! {"_id": &self.id})
            .await?
            .ok_or(V1Error::NoSuchUser)?;

        if account.email != self.email {
            return Err(V1Error::EmailMismatch.into());
        }

        if account.verified {
            return Ok(());
        }

        account.verified = true;
        account.save_replace(accounts).await?;

        Ok(())
    }

    fn peek(&self, _id: &str, expiry: u64) -> Option<Box<dyn Peekable>> {
        Some(Box::new(ServiceTriggerWrapper(ServicesTriggers {
            expiry,
            value: ServicesTriggerTypes::EmailVerification {
                email: self.email.clone(),
                username: self.username.clone(),
                id: self.id,
            },
        })))
    }
}
