use actix_multipart::Multipart;
use actix_web::{web::Path, *};
use std::{error::Error, path::PathBuf};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

use goodmorning_bindings::services::v1::{V1Error, V1Response};

use crate::{functions::*, structs::*, traits::CollectionItem, *};

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
    let mut account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    let mut user_path = PathBuf::from(path.trim_start_matches('/'));

    if let [_, "Shared", user, ..] = user_path
        .iter()
        .map(|s| s.to_str().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
    {
        account = if let Some(account) = Account::find_by_username(user.to_string()).await? {
            account.v1_restrict_verified()?
        } else {
            return Err(V1Error::FileNotFound.into());
        };
        user_path = [user_path.iter().next().unwrap()]
            .into_iter()
            .chain(user_path.iter().skip(3))
            .collect();
    }

    if !editable(&user_path, &account.services) || has_dotdot(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }

    let path_buf = get_user_dir(account.id, None).join(user_path);

    if fs::try_exists(&path_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    let size = req
        .headers()
        .get("content-length")
        .unwrap()
        .to_str()?
        .parse::<u64>()?;

    if account
        .exceeds_limit(STORAGE_LIMITS.get().unwrap(), Some(size), None)
        .await?
    {
        return Err(V1Error::StorageFull.into());
    }

    let data = bytes_from_multipart(payload).await?;

    file_check_v1(&data, &path_buf)?;

    account.stored.as_mut().unwrap().value += size;
    account.save_replace(ACCOUNTS.get().unwrap()).await?;

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
    let mut account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    let mut user_path = PathBuf::from(path.trim_start_matches('/'));

    if let [_, "Shared", user, ..] = user_path
        .iter()
        .map(|s| s.to_str().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
    {
        account = if let Some(account) = Account::find_by_username(user.to_string()).await? {
            account.v1_restrict_verified()?
        } else {
            return Err(V1Error::FileNotFound.into());
        };
        user_path = [user_path.iter().next().unwrap()]
            .into_iter()
            .chain(user_path.iter().skip(3))
            .collect();
    }

    if !editable(&user_path, &account.services) || has_dotdot(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }

    let path_buf = get_user_dir(account.id, None).join(user_path);

    if fs::try_exists(&path_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    let size = req
        .headers()
        .get("content-length")
        .unwrap()
        .to_str()?
        .parse::<u64>()?;

    let to_metedata = if fs::try_exists(&path_buf).await? {
        Some(fs::metadata(&path_buf).await?)
    } else {
        None
    };

    let to_size = match &to_metedata {
        Some(meta) if meta.is_file() => meta.len(),
        Some(_) => dir_size(&path_buf).await?,
        None => 0,
    };

    if account
        .exceeds_limit(STORAGE_LIMITS.get().unwrap(), Some(size), Some(to_size))
        .await?
    {
        return Err(V1Error::StorageFull.into());
    }

    let data = bytes_from_multipart(payload).await?;

    file_check_v1(&data, &path_buf)?;

    let parent = path_buf.parent().unwrap();

    if let Some(meta) = to_metedata {
        if meta.is_dir() {
            fs::remove_dir_all(&path_buf).await?;
        } else {
            fs::remove_file(&path_buf).await?;
        }
    }

    let stored = account.stored.as_mut().unwrap();

    stored.value += size;
    stored.value = stored.value.saturating_sub(to_size);
    account.save_replace(ACCOUNTS.get().unwrap()).await?;

    if !fs::try_exists(parent).await? {
        fs::create_dir_all(parent).await?;
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
