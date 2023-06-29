use std::error::Error;

use goodmorning_bindings::{
    services::v1::V1Error,
    structs::{ProfileAccount, ProfileCustomisable},
};
use mongodb::bson;
use tokio::{fs, io::AsyncWriteExt};

use crate::structs::Account;

use super::get_usersys_dir;

pub fn validate_profile(profile: &ProfileCustomisable) -> Result<(), V1Error> {
    if profile.details.len() > 20 {
        return Err(V1Error::TooManyProfileDetails);
    }

    if profile.description.len() > 2000 {
        return Err(V1Error::ExceedsMaximumLength);
    }

    for (i, detail) in profile.details.iter().enumerate() {
        if !detail.validate() {
            return Err(V1Error::InvalidDetail { index: i as u8 });
        }
    }

    Ok(())
}

pub async fn save_profile(
    profile: &ProfileCustomisable,
    id: i64,
    service: &str,
) -> Result<(), Box<dyn Error>> {
    let path = get_usersys_dir(id, Some(service)).join("profile.bson");

    if !fs::try_exists(path.parent().unwrap()).await? {
        return Err(V1Error::NotCreated.into());
    }

    let doc = bson::to_document(&profile)?;
    let mut bytes = Vec::new();
    doc.to_writer(&mut bytes)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .await?;
    file.write_all(&bytes).await?;

    Ok(())
}

pub async fn reset_profile(id: i64, service: &str) -> Result<(), Box<dyn Error>> {
    let path = get_usersys_dir(id, Some(service)).join("profile.bson");

    Ok(fs::remove_file(path).await?)
}

pub async fn read_profile(id: i64, service: &str) -> Result<ProfileCustomisable, Box<dyn Error>> {
    let path = get_usersys_dir(id, Some(service)).join("profile.bson");

    if !fs::try_exists(path.parent().unwrap()).await? {
        return Err(V1Error::NotCreated.into());
    }

    if !fs::try_exists(&path).await? {
        return Ok(ProfileCustomisable::default());
    }

    let bytes = fs::read(path).await?;
    let bson = bson::from_slice(&bytes)?;
    let profile = bson::from_bson(bson)?;
    Ok(profile)
}

pub fn to_profile_acccount(account: Account) -> ProfileAccount {
    ProfileAccount {
        id: account.id,
        username: account.username,
        verified: account.verified,
        created: account.created,
        status: account.status,
    }
}
