use goodmorning_bindings::services::v1::V1IdentifierType;
use std::path::PathBuf;

use chrono::Utc;
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};
use tokio::io;

use crate::{functions::*, structs::*, traits::*, *};

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    password_hash: String,
    pub token: String,

    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: String,
    pub username: String,

    pub email: String,
    pub verified: bool,

    pub last_seen: u64,
    pub created: u64,
}

impl Account {
    /// Create new instance of self with default values, including a unique ID
    pub fn new(username: String, password: &str, email: &str) -> Self {
        let now = Utc::now().timestamp() as u64;
        let id = hex::encode(fastrand::u128(..).to_be_bytes());

        Self {
            password_hash: Self::hash_with_id(password, &id),
            token: Self::token(),

            id,
            username,

            email: email.to_lowercase(),
            verified: false,

            last_seen: now,
            created: now,
        }
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
            id: self.id.clone(),
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
    ) -> Result<Option<Self>, mongodb::error::Error> {
        match identifier_type {
            IdentifierType::Id => Account::find_by_id(&identifier, collection).await,
            IdentifierType::Email => Account::find_by_email(&identifier, collection).await,
            IdentifierType::Username => Account::find_by_username(identifier, collection).await,
        }
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
                dir_size(&PathBuf::from(&format!("{}/{}", USERCONTENT.as_str(), self.id))).await?
                    + extra.unwrap_or_default()
                    - remove.unwrap_or_default()
                    > self.storage_limits(limits),
            )
        } else {
            Ok(true)
        }
    }
}

impl Account {
    fn hash_with_id(password: &str, id: &str) -> String {
        hash(password, vec![id])
    }

    fn hash(&self, password: &str) -> String {
        Self::hash_with_id(password, &self.id)
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

impl CollectionItem for Account {
    fn id(&self) -> &str {
        &self.id
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

#[derive(Clone, Copy)]
pub struct StorageLimits {
    pub _1: u64,
}
