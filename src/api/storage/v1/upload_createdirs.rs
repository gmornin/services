use actix_multipart::Multipart;
use actix_web::{web::Path, *};
use std::{error::Error, path::PathBuf};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

use goodmorning_bindings::services::v1::{V1Error, V1Response};

use crate::{functions::*, structs::*, *};

#[post("/upload-createdirs/{token}/{path:.*}")]
pub async fn upload_createdirs(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> HttpResponse {
    from_res(upload_createdirs_task(payload, path, req).await)
}

async fn upload_createdirs_task(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> Result<V1Response, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    let user_path = PathBuf::from(path.trim_start_matches('/'));

    if !editable(&user_path, &account.services) || has_dotdot(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }

    let path_buf = get_user_dir(account.id, None).join(user_path);

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
        return Err(V1Error::StorageFull.into());
    }

    let data = bytes_from_multipart(payload).await?;

    file_check_v1(&data, &path_buf)?;

    let parent = path_buf.parent().unwrap();

    if !fs::try_exists(parent).await? {
        fs::create_dir_all(parent).await?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path_buf)
        .await?;

    file.write_all(&data).await?;

    Ok(V1Response::FileItemCreated)
}

#[post("/upload-createdirs-overwrite/{token}/{path:.*}")]
pub async fn upload_createdirs_overwrite(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> HttpResponse {
    from_res(upload_createdirs_overwrite_task(payload, path, req).await)
}

async fn upload_createdirs_overwrite_task(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> Result<V1Response, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    let user_path = PathBuf::from(path.trim_start_matches('/'));

    if !editable(&user_path, &account.services) || has_dotdot(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }

    let path_buf = get_user_dir(account.id, None).join(user_path);

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
        return Err(V1Error::StorageFull.into());
    }

    let data = bytes_from_multipart(payload).await?;

    file_check_v1(&data, &path_buf)?;

    let parent = path_buf.parent().unwrap();

    if !fs::try_exists(parent).await? {
        fs::create_dir_all(parent).await?;
    } else if fs::try_exists(&path_buf).await? && fs::metadata(&path_buf).await?.is_dir() {
        fs::remove_dir(&path_buf).await?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&path_buf)
        .await?;

    file.write_all(&data).await?;

    Ok(V1Response::FileItemCreated)
}
