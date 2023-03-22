use actix_multipart::Multipart;
use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::{error::Error, path::PathBuf};
use tokio::{fs::{OpenOptions, self}, io::AsyncWriteExt};

use super::*;
use crate::{functions::{get_accounts, bytes_from_multipart}, structs::Account};

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
) -> Json<Responses> {
    Json(match overwrite_task(payload, &path.path, &path.token, &db).await {
        Ok(res) => res,
        Err(e) => {
            Responses::Error { kind: ErrorKind::External(e.to_string()) }
        },
    })
}

async fn overwrite_task(payload: Multipart, path: &str, token: &str, db: &Database) -> Result<Responses, Box<dyn Error>> {
    let accounts = get_accounts(&db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => return Ok(Responses::Error { kind: ErrorKind::InvalidToken }),
    };

    let path_buf = PathBuf::from(format!("usercontent/{}/{}", account.id, path));

    if !path_buf.exists() {
        return Err(ErrorKind::NotFound.into());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .open(&path_buf)
        .await?;

    let data = bytes_from_multipart(payload).await?;

    if let Err(e) = file.write_all(&data).await {
        return Ok(Responses::Error { kind: ErrorKind::FsError(e.to_string()) })
    }

    Ok(Responses::Overwritten {
        path: format!("/{}/{}", account.id, path),
    })
}
