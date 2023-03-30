use actix_multipart::Multipart;
use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::{error::Error, path::PathBuf};
use tokio::{
    fs::{try_exists, OpenOptions},
    io::AsyncWriteExt,
};

use crate::{
    api::v1::*,
    functions::{bytes_from_multipart, get_accounts, to_res},
    structs::{Account, StorageLimits, Visibilities},
};

#[derive(Deserialize)]
struct StaticPath {
    path: String,
    token: String,
}

#[post("/write_new/{path:.*}")]
pub async fn write_new(
    payload: Multipart,
    path: Path<StaticPath>,
    req: HttpRequest,
    db: Data<Database>,
    storage_limits: Data<StorageLimits>,
) -> Json<GMResponses> {
    Json(to_res(
        write_new_task(payload, &path.path, &path.token, req, &db, &storage_limits).await,
    ))
}

async fn write_new_task(
    payload: Multipart,
    path: &str,
    token: &str,
    req: HttpRequest,
    db: &Database,
    storage_limits: &StorageLimits,
) -> Result<GMResponses, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(GMResponses::Error {
                kind: GMError::InvalidToken,
            })
        }
    };

    let path_buf = PathBuf::from(format!("usercontent/{}/{}", account.id, path));

    if try_exists(&path_buf).await? {
        return Err(GMError::PathOccupied.into());
    }

    if account
        .exceeds_limit(
            storage_limits,
            Some(
                req.headers()
                    .get("content-length")
                    .unwrap()
                    .to_str()?
                    .parse::<u64>()?,
            ),
            None,
        )
        .await?
    {
        return Err(GMError::FileTooLarge.into());
    }

    Visibilities::check_all_dirs(&path_buf.parent().unwrap()).await?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path_buf)
        .await?;

    let data = bytes_from_multipart(payload).await?;

    file.write_all(&data).await?;

    Ok(GMResponses::Overwritten {
        path: format!("/{}/{}", account.id, path),
    })
}
