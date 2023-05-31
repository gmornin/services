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

use crate::{functions::*, structs::*};

use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
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
) -> Json<V1Response> {
    Json(V1Response::from_res(
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
) -> Result<V1Response, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(V1Response::Error {
                kind: V1Error::InvalidToken,
            })
        }
    };

    let path_buf = PathBuf::from(format!("usercontent/{}/{}", account.id, path));

    if !editable(&path_buf) {
        return Err(V1Error::NotEditable.into());
    }

    if try_exists(&path_buf).await? {
        return Err(V1Error::PathOccupied.into());
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
        return Err(V1Error::FileTooLarge.into());
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path_buf)
        .await?;

    let data = bytes_from_multipart(payload).await?;

    file.write_all(&data).await?;

    Ok(V1Response::FileItemCreated {
        path: format!("/{}/{}", account.id, path),
    })
}
