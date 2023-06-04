use actix_multipart::Multipart;
use actix_web::{
    web::{Json, Path},
    *,
};
use std::{error::Error, path::PathBuf};
use tokio::{
    fs::{self, try_exists, OpenOptions},
    io::AsyncWriteExt,
};

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
};

#[post("/write-new/{token}/{path:.*}")]
pub async fn write_new(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> Json<V1Response> {
    let (token, path) = path.into_inner();
    Json(V1Response::from_res(
        write_new_task(payload, &path, &token, req).await,
    ))
}

async fn write_new_task(
    payload: Multipart,
    path: &str,
    token: &str,
    req: HttpRequest,
) -> Result<V1Response, Box<dyn Error>> {
    let accounts = get_accounts(DATABASE.get().unwrap());
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(V1Response::Error {
                kind: V1Error::InvalidToken,
            })
        }
    };

    if !account.verified {
        return Ok(V1Response::Error {
            kind: V1Error::NotVerified,
        });
    }

    let path_buf = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(&account.id)
        .join(path.trim_start_matches('/'));

    if !fs::try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if try_exists(&path_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    if account
        .exceeds_limit(
            STORAGE_LIMITS.get().unwrap(),
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
