use actix_multipart::Multipart;
use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::{error::Error, path::PathBuf};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::{
    api::v1::*,
    functions::{bytes_from_multipart, file_size, get_accounts, to_res},
    structs::{Account, StorageLimits},
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
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(GMResponses::Error {
                kind: GMError::InvalidToken,
            })
        }
    };

    let path_buf = PathBuf::from(format!("usercontent/{}/{}", account.id, path));
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
            Some(file_size(&path_buf).await?),
        )
        .await?
    {
        return Err(GMError::FileTooLarge.into());
    }

    if !path_buf.exists() {
        return Err(GMError::FileNotFound.into());
    }

    let mut file = OpenOptions::new().write(true).open(&path_buf).await?;

    let data = bytes_from_multipart(payload).await?;

    if let Err(e) = file.write_all(&data).await {
        return Ok(GMResponses::Error {
            kind: GMError::FsError {
                content: e.to_string(),
            },
        });
    }

    Ok(GMResponses::Overwritten {
        path: format!("/{}/{}", account.id, path),
    })
}
