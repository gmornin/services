use std::{error::Error, path::PathBuf};

use goodmorning_bindings::{services::v1::V1Error, structs::Profile};
use mongodb::bson;
use tokio::{fs, io::AsyncWriteExt};

use crate::USERCONTENT;

pub fn validate_profile(profile: &Profile) -> Result<(), V1Error> {
    if profile.details.len() > 20 {
        return Err(V1Error::TooManyProfileDetails);
    }

    if profile.description.len() > 2000 {
        return Err(V1Error::ExceedsMaximumLength);
    }
    Ok(())
}

pub async fn save_profile(profile: &Profile, id: i64, service: &str) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(id.to_string())
        .join(service)
        .join(".system")
        .join("profile.bson");

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

pub async fn read_profile(id: i64, service: &str) -> Result<Profile, Box<dyn Error>> {
    let path = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(id.to_string())
        .join(service)
        .join(".system")
        .join("profile.bson");

    if !fs::try_exists(path.parent().unwrap()).await? {
        return Err(V1Error::NotCreated.into());
    }

    if !fs::try_exists(&path).await? {
        return Ok(Profile::default());
    }

    let bytes = fs::read(path).await?;
    let bson = bson::from_slice(&bytes)?;
    let profile = bson::from_bson(bson)?;
    Ok(profile)
}
