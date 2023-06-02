use actix_multipart::Multipart;
use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use std::{error::Error, path::PathBuf};
use tokio::{
    fs::{try_exists, OpenOptions},
    io::AsyncWriteExt,
};

use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*};

#[post("/overwrite/{token}/{path:.*}")]
pub async fn overwrite(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
    db: Data<Database>,
    storage_limits: Data<StorageLimits>,
) -> Json<V1Response> {
    let (token, path) = path.into_inner();
    Json(V1Response::from_res(
        overwrite_task(payload, &path, &token, req, &db, &storage_limits).await,
    ))
}

async fn overwrite_task(
    payload: Multipart,
    path: &str,
    token: &str,
    req: HttpRequest,
    db: &Database,
    storage_limits: &StorageLimits,
) -> Result<V1Response, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await.unwrap() {
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

    if !try_exists(&path_buf).await.unwrap() {
        return Err(V1Error::FileNotFound.into());
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
        return Err(V1Error::FileTooLarge.into());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .open(&path_buf)
        .await
        .unwrap();

    let data = bytes_from_multipart(payload).await.unwrap();

    file.write_all(&data).await.unwrap();

    Ok(V1Response::Overwritten {
        path: format!("/{}/{}", account.id, path),
    })
}
