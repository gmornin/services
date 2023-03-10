use crate::{
    functions::get_accounts,
    structs::GMError,
    traits::{CollectionItem, Triggerable},
};
use async_trait::async_trait;
use lettre::{
    message::{header::ContentType, MessageBuilder},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmailVerification {
    pub email: String,
    pub username: String,
    pub id: String,
}

#[typetag::serde]
#[async_trait]
impl Triggerable for EmailVerification {
    async fn init(&self, _db: &Database) -> Result<(), Box<dyn Error>> {
        let email_username = env::var("SMTP_USERNAME").expect("`SMTP_USERNAME` not found in env");
        let email_password = env::var("SMTP_PASSWORD").expect("`SMTP_PASSWORD` not found in env");
        let smtp_relay = env::var("SMTP_RELAY").expect("`SMTP_RELAY` not found in env");
        let smtp_from = env::var("SMTP_FROM").expect("`SMTP_FROM` not found in env");

        let message = MessageBuilder::new()
            .from(smtp_from.parse()?)
            .to(format!("{}<{}>", self.username, self.email).parse()?)
            .subject("Verify your GoodMorning account")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                "{}{}",
                env::var("TRIGGER_URL").expect("cannot find `TRIGGER_URL` in env"),
                self.id
            ))?;

        let creds = Credentials::new(email_username, email_password);

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_relay)?
            .credentials(creds)
            .build();

        mailer.send(message).await?;

        Ok(())
    }

    async fn trigger(&self, db: &Database) -> Result<(), Box<dyn Error>> {
        let accounts = get_accounts(db);
        let mut account = accounts
            .find_one(doc! {"_id": &self.id}, None)
            .await?
            .ok_or(GMError::UserNotFound)?;

        if account.email != self.email {
            return Err(GMError::EmailMismatch.into());
        }

        if account.verified {
            return Ok(());
        }

        account.verified = true;
        account.save_replace(&accounts).await?;

        Ok(())
    }
}
