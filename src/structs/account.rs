use goodmorning_bindings::services::v1::{V1Error, V1IdentifierType};
use serde_inline_default::serde_inline_default;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use chrono::Utc;
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};

use crate::{
    functions::*, structs::*, traits::*, ACCOUNTS, DATABASE, EMAIL_VERIFICATION_DURATION,
    EMAIL_WHITELIST, STORAGE_SIZE_RECHECK, TOKEN_LENGTH, TRIGGERS, USERNAME_MAX, USERNAME_MIN,
    VERIFICATION,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageSize {
    pub last_checked: u64,
    pub value: u64,
}

impl StorageSize {
    pub fn new(value: u64) -> Self {
        Self {
            last_checked: Utc::now().timestamp() as u64,
            value,
        }
    }

    pub fn set(&mut self, value: u64) {
        self.value = value;
        self.last_checked = Utc::now().timestamp() as u64;
    }

    pub fn outdated(&self) -> bool {
        self.last_checked + STORAGE_SIZE_RECHECK.get().unwrap() < Utc::now().timestamp() as u64
    }

    pub async fn update(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
        self.set(dir_size(&get_user_dir(id, None)).await?);
        Ok(())
    }
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    password_hash: String,
    pub token: String,

    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: i64,
    pub username: String,

    pub email: String,
    #[serde(default)]
    pub last_verify: u64,
    pub verified: bool,

    #[serde(default)]
    pub status: String,
    pub created: u64,

    // GMServices
    #[serde(default)]
    pub services: Vec<String>,

    #[serde(default)]
    pub counters: HashMap<String, i64>,

    pub stored: Option<StorageSize>,

    // AccessType
    #[serde(default)]
    pub access: HashMap<String, HashSet<i64>>,

    #[serde_inline_default("base".to_string())]
    pub limit: String,
}

impl Account {
    pub async fn get_stored(&mut self) -> Result<u64, Box<dyn Error>> {
        let updated = match &mut self.stored {
            Some(stored) if stored.outdated() => {
                stored.update(self.id).await?;
                true
            }
            Some(_) => false,
            None => {
                self.stored = Some(StorageSize::new(
                    dir_size(&get_user_dir(self.id, None)).await?,
                ));
                true
            }
        };

        if updated {
            self.save_replace(ACCOUNTS.get().unwrap()).await?;
        }

        Ok(self.stored.as_ref().unwrap().value)
    }

    pub async fn get_stored_nosave(&mut self) -> Result<u64, Box<dyn Error>> {
        match &mut self.stored {
            Some(stored) if stored.outdated() => stored.update(self.id).await?,
            Some(_) => {}
            None => {
                self.stored = Some(StorageSize::new(
                    dir_size(&get_user_dir(self.id, None)).await?,
                ))
            }
        }

        Ok(self.stored.as_ref().unwrap().value)
    }

    pub fn get_counter(&mut self, label: String) -> &mut i64 {
        self.counters.entry(label).or_insert(1)
    }

    pub fn matches_identifier(&self, ident: &str, ident_type: IdentifierType) -> bool {
        match ident_type {
            IdentifierType::Id => self.id.to_string().as_str() == ident,
            IdentifierType::Email => self.email.eq_ignore_ascii_case(ident),
            IdentifierType::Username => self.username.eq_ignore_ascii_case(ident),
        }
    }

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
            last_verify: now,
            verified: false,

            status: String::new(),
            created: now,

            services: Vec::new(),

            counters: HashMap::new(),

            stored: Some(StorageSize::new(0)),

            access: HashMap::default(),
            limit: "base".to_string(),
        })
    }

    /// Checks if password matches
    pub fn password_matches(&self, password: &str) -> bool {
        self.hash(password) == self.password_hash
    }

    pub fn is_email_valid(email: &str) -> bool {
        email
            .split_once('@')
            .is_some_and(|(_, dom)| EMAIL_WHITELIST.get().unwrap().allow(dom))
    }

    /// Creates an `EmailVerification` instance
    pub async fn email_verification(&self) -> Result<(), Box<dyn Error>> {
        if !VERIFICATION.get().unwrap() {
            return Ok(());
        }

        let trigger_item = EmailVerification {
            username: self.username.clone(),
            email: self.email.clone(),
            id: self.id,
        };
        let trigger = Trigger::new_from_action(
            Box::new(trigger_item),
            EMAIL_VERIFICATION_DURATION.get().unwrap(),
        );
        trigger.init(DATABASE.get().unwrap()).await?;
        trigger.save_create(TRIGGERS.get().unwrap()).await?;

        Ok(())
    }

    /// regenerates token
    pub fn regeneratetoken(&mut self) {
        self.token = Self::token()
    }

    /// update password from &mut self
    pub fn change_password(&mut self, new: &str) {
        self.password_hash = Self::hash_with_id(new, &self.id.to_string());
    }

    pub fn username_valid(s: &str) -> bool {
        s.len() <= *USERNAME_MAX.get().unwrap()
            && s.len() >= *USERNAME_MIN.get().unwrap()
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
    }
}

impl Account {
    pub async fn find_by_username(username: String) -> Result<Option<Self>, mongodb::error::Error> {
        ACCOUNTS
            .get()
            .unwrap()
            .find_one(doc! {"username":  case_insensitive(username)})
            .await
    }

    pub async fn find_by_email(email: &str) -> Result<Option<Self>, mongodb::error::Error> {
        ACCOUNTS
            .get()
            .unwrap()
            .find_one(doc! {"email": email.to_lowercase()})
            .await
    }

    pub async fn find_by_token(token: &str) -> Result<Option<Self>, mongodb::error::Error> {
        ACCOUNTS
            .get()
            .unwrap()
            .find_one(doc! {"token": token})
            .await
    }

    pub async fn find_by_idenifier(
        identifier_type: &IdentifierType,
        identifier: String,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        Ok(match identifier_type {
            IdentifierType::Id => {
                Account::find_by_id(identifier.parse()?, ACCOUNTS.get().unwrap()).await?
            }
            IdentifierType::Email => Account::find_by_email(&identifier).await?,
            IdentifierType::Username => Account::find_by_username(identifier).await?,
        })
    }
}

impl Account {
    pub async fn v1_get_by_token(token: &str) -> Result<Self, Box<dyn Error>> {
        match Account::find_by_token(token).await? {
            Some(acc) => Ok(acc),
            None => Err(V1Error::InvalidToken.into()),
        }
    }

    pub async fn v1_get_by_id(id: i64) -> Result<Self, Box<dyn Error>> {
        match Account::find_by_id(id, ACCOUNTS.get().unwrap()).await? {
            Some(acc) => Ok(acc),
            None => Err(V1Error::NoSuchUser.into()),
        }
    }

    pub async fn v1_get_by_username(username: String) -> Result<Self, Box<dyn Error>> {
        match Account::find_by_username(username).await? {
            Some(acc) => Ok(acc),
            None => Err(V1Error::NoSuchUser.into()),
        }
    }

    pub fn v1_restrict_verified(self) -> Result<Self, V1Error> {
        if !self.verified && *VERIFICATION.get().unwrap() {
            return Err(V1Error::NotVerified);
        }

        Ok(self)
    }

    pub fn v1_contains(self, service: &GMServices) -> Result<Self, V1Error> {
        if !self.services.contains(&service.as_str().to_string()) {
            return Err(V1Error::NotCreated);
        }

        Ok(self)
    }

    pub fn v1_not_contains(self, service: &GMServices) -> Result<Self, V1Error> {
        if self.services.contains(&service.as_str().to_string()) {
            return Err(V1Error::AlreadyCreated);
        }

        Ok(self)
    }
}

impl Account {
    pub fn storage_limits(&self, limits: &StorageLimitConfigs) -> u64 {
        *limits.0.get(&self.limit).unwrap_or(&0)
    }

    pub async fn exceeds_limit(
        &mut self,
        limits: &StorageLimitConfigs,
        extra: Option<u64>,
        remove: Option<u64>,
    ) -> Result<bool, Box<dyn Error>> {
        if extra > remove {
            Ok(
                self.get_stored().await? + extra.unwrap_or_default() - remove.unwrap_or_default()
                    > self.storage_limits(limits),
            )
        } else {
            Ok(false)
        }
    }

    pub async fn exceeds_limit_nosave(
        &mut self,
        limits: &StorageLimitConfigs,
        extra: Option<u64>,
        remove: Option<u64>,
    ) -> Result<bool, Box<dyn Error>> {
        if extra > remove {
            Ok(self.get_stored_nosave().await? + extra.unwrap_or_default()
                - remove.unwrap_or_default()
                > self.storage_limits(limits))
        } else {
            Ok(false)
        }
    }
}

impl Account {
    fn hash_with_id(password: &str, id: &str) -> String {
        hash(password, id)
    }

    fn hash(&self, password: &str) -> String {
        Self::hash_with_id(password, &self.id.to_string())
    }

    fn token() -> String {
        (0..*TOKEN_LENGTH.get().unwrap())
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
