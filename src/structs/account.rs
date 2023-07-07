use goodmorning_bindings::services::v1::{V1Error, V1IdentifierType};
use std::error::Error;

use chrono::Utc;
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};
use tokio::io;

use crate::{functions::*, structs::*, traits::*, ACCOUNTS};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    password_hash: String,
    pub token: String,

    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: i64,
    pub username: String,

    pub email: String,
    pub verified: bool,

    #[serde(default)]
    pub status: String,
    pub created: u64,

    #[serde(default)]
    pub services: Vec<GMServices>,
}

impl Account {
    /// Create new instance of self with default values, including a unique ID
    pub async fn new(
        username: String,
        password: &str,
        email: &str,
        db: &Database,
    ) -> Result<Self, Box<dyn Error>> {
        let now = Utc::now().timestamp() as u64;
        let id = Counter::bump_get("id_counter", db).await?;

        Ok(Self {
            password_hash: Self::hash_with_id(password, &id.to_string()),
            token: Self::token(),

            id,
            username,

            email: email.to_lowercase(),
            verified: false,

            status: String::new(),
            created: now,

            services: Vec::new(),
        })
    }

    /// Checks if password matches
    pub fn password_matches(&self, password: &str) -> bool {
        self.hash(password) == self.password_hash
    }

    /// Creates an `EmailVerification` instance
    pub fn email_verification(&self) -> EmailVerification {
        EmailVerification {
            username: self.username.clone(),
            email: self.email.clone(),
            id: self.id,
        }
    }

    /// regenerates token
    pub fn regeneratetoken(&mut self) {
        self.token = Self::token()
    }
}

impl Account {
    pub async fn find_by_username(
        username: String,
        collection: &Collection<Self>,
    ) -> Result<Option<Self>, mongodb::error::Error> {
        collection
            .find_one(doc! {"username": case_insensitive(username)}, None)
            .await
    }

    pub async fn find_by_email(
        email: &str,
        collection: &Collection<Self>,
    ) -> Result<Option<Self>, mongodb::error::Error> {
        collection
            .find_one(doc! {"email": email.to_lowercase()}, None)
            .await
    }

    pub async fn find_by_token(
        token: &str,
        collection: &Collection<Self>,
    ) -> Result<Option<Self>, mongodb::error::Error> {
        collection.find_one(doc! {"token": token}, None).await
    }

    pub async fn find_by_idenifier(
        identifier_type: &IdentifierType,
        identifier: String,
        collection: &Collection<Self>,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        Ok(match identifier_type {
            IdentifierType::Id => Account::find_by_id(identifier.parse()?, collection).await?,
            IdentifierType::Email => Account::find_by_email(&identifier, collection).await?,
            IdentifierType::Username => Account::find_by_username(identifier, collection).await?,
        })
    }
}

impl Account {
    pub async fn v1_get_by_token(token: &str) -> Result<Self, Box<dyn Error>> {
        match Account::find_by_token(token, ACCOUNTS.get().unwrap()).await? {
            Some(acc) => Ok(acc),
            None => Err(V1Error::InvalidToken.into()),
        }
    }

    pub async fn v1_get_by_id(id: i64) -> Result<Self, Box<dyn Error>> {
        match Account::find_by_id(id, ACCOUNTS.get().unwrap()).await? {
            Some(acc) => Ok(acc),
            None => Err(V1Error::InvalidToken.into()),
        }
    }

    pub fn v1_restrict_verified(self) -> Result<Self, V1Error> {
        if !self.verified {
            return Err(V1Error::NotVerified);
        }

        Ok(self)
    }

    pub fn v1_contains(self, service: &GMServices) -> Result<Self, V1Error> {
        if !self.services.contains(service) {
            return Err(V1Error::NotCreated);
        }

        Ok(self)
    }

    pub fn v1_not_contains(self, service: &GMServices) -> Result<Self, V1Error> {
        if self.services.contains(service) {
            return Err(V1Error::AlreadyCreated);
        }

        Ok(self)
    }
}

impl Account {
    pub fn storage_limits(&self, limits: &StorageLimits) -> u64 {
        limits._1
    }

    pub async fn exceeds_limit(
        &self,
        limits: &StorageLimits,
        extra: Option<u64>,
        remove: Option<u64>,
    ) -> io::Result<bool> {
        if extra > remove {
            Ok(
                dir_size(&get_user_dir(self.id, None)).await? + extra.unwrap_or_default()
                    - remove.unwrap_or_default()
                    > self.storage_limits(limits),
            )
        } else {
            Ok(false)
        }
    }
}

impl Account {
    fn hash_with_id(password: &str, id: &str) -> String {
        hash(password, vec![id])
    }

    fn hash(&self, password: &str) -> String {
        Self::hash_with_id(password, &self.id.to_string())
    }

    fn token() -> String {
        const TOKEN_LENGTH: u8 = 4;

        (0..TOKEN_LENGTH)
            .map(|_| Self::token_section())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn token_section() -> String {
        hex::encode(fastrand::u128(..).to_be_bytes())
    }
}

impl CollectionItem<i64> for Account {
    fn id(&self) -> i64 {
        self.id
    }
}

#[derive(Serialize, Deserialize)]
pub enum IdentifierType {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "username")]
    Username,
}

impl From<V1IdentifierType> for IdentifierType {
    fn from(value: V1IdentifierType) -> Self {
        match value {
            V1IdentifierType::Email => Self::Email,
            V1IdentifierType::Username => Self::Username,
            V1IdentifierType::Id => Self::Id,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StorageLimits {
    pub _1: u64,
}
