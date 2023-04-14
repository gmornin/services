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
    api::services::v1::*,
    functions::*,
    structs::*,
};

#[derive(Deserialize)]
struct StaticPath {
    path: String,
    token: String,
}

#[post("/overwrite/{path:.*}")]
pub async fn overwrite(
    payload: Multipart,
    path: Path<StaticPath>,
    req: HttpRequest,
    db: Data<Database>,
    storage_limits: Data<StorageLimits>,
) -> Json<GMResponses> {
    Json(to_res(
        overwrite_task(payload, &path.path, &path.token, req, &db, &storage_limits).await,
    ))
}

async fn overwrite_task(
    payload: Multipart,
    path: &str,
    token: &str,
    req: HttpRequest,
    db: &Database,
    storage_limits: &StorageLimits,
) -> Result<GMResponses, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await.unwrap() {
        Some(account) => account,
        None => {
            return Ok(GMResponses::Error {
                kind: GMError::InvalidToken,
            })
        }
    };

    let path_buf = PathBuf::from(format!("usercontent/{}/{}", account.id, path));

    if !try_exists(&path_buf).await.unwrap() {
        return Err(GMError::FileNotFound.into());
    }

    if account
        .exceeds_limit(
            storage_limits,
            Some(
                req.headers()
                    .get("content-length")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
            ),
            Some(file_size(&path_buf).await.unwrap()),
        )
        .await
        .unwrap()
    {
        return Err(GMError::FileTooLarge.into());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .open(&path_buf)
        .await
        .unwrap();

    let data = bytes_from_multipart(payload).await.unwrap();

    file.write_all(&data).await.unwrap();

    Ok(GMResponses::Overwritten {
        path: format!("/{}/{}", account.id, path),
    })
}
