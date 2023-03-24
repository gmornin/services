use actix_multipart::Multipart;
use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::{error::Error, path::PathBuf};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::{functions::{get_accounts, bytes_from_multipart, to_res}, structs::Account, api::v1::*};

#[derive(Deserialize)]
struct StaticPath {
    path: String,
    token: String,
}

#[post("/overwrite/{token}/{path:.*}")]
pub async fn overwrite(
    payload: Multipart,
    path: Path<StaticPath>,
    db: Data<Database>,
) -> Json<GMResponses> {
    Json(to_res(overwrite_task(payload, &path.path, &path.token, &db).await))
}

async fn overwrite_task(payload: Multipart, path: &str, token: &str, db: &Database) -> Result<GMResponses, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => return Ok(GMResponses::Error { kind: GMError::InvalidToken }),
    }; 

    let path_buf = PathBuf::from(format!("usercontent/{}/{}", account.id, path));

    if !path_buf.exists() {
        return Err(GMError::FileNotFound.into());
    } 

    let mut file = OpenOptions::new()
        .write(true)
        .open(&path_buf)
        .await?;

    let data = bytes_from_multipart(payload).await?;

    if let Err(e) = file.write_all(&data).await {
        return Ok(GMResponses::Error { kind: GMError::FsError(e.to_string()) })
    } 

    Ok(GMResponses::Overwritten {
        path: format!("/{}/{}", account.id, path),
    })
}
