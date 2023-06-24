use actix_multipart::Multipart;
use actix_web::{web::Path, *};
use std::error::Error;
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::services::v1::{V1Error, V1Response};

#[post("/upload/{token}/{path:.*}")]
pub async fn upload(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> HttpResponse {
    from_res(upload_task(payload, path, req).await)
}

async fn upload_task(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> Result<V1Response, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let accounts = ACCOUNTS.get().unwrap();
    let account = match Account::find_by_token(&token, accounts).await? {
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

    let path_buf = get_user_dir(account.id, None).join(path.trim_start_matches('/'));

    if !editable(&path_buf) || has_dotdot(&path_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if fs::try_exists(&path_buf).await? {
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

    let expected = MIME_DB
        .get()
        .unwrap()
        .get_mime_types_from_file_name(path_buf.file_name().unwrap().to_str().unwrap());
    match MIME_DB.get().unwrap().get_mime_type_for_data(&data) {
        Some((mime, _)) if !expected.is_empty() && !expected.contains(&mime) => {
            return Err(V1Error::FileTypeMismatch {
                expected: expected[0].to_string(),
                got: mime.to_string(),
            }
            .into());
        }
        _ => {}
    }

    file.write_all(&data).await?;

    Ok(V1Response::FileItemCreated)
}
