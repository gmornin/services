use std::error::Error;
use std::path::Path;

use mongodb::bson;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn bson_read<T: DeserializeOwned + Default>(path: &Path) -> Result<T, Box<dyn Error>> {
    if !fs::try_exists(path).await? {
        return Ok(T::default());
    }

    let bytes = fs::read(path).await?;
    let bson = bson::from_slice(&bytes)?;
    Ok(bson::from_bson(bson)?)
}

pub async fn bson_write<T: Serialize>(doc: &T, path: &Path) -> Result<(), Box<dyn Error>> {
    let doc = bson::to_document(doc)?;
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
