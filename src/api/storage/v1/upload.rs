use actix_multipart::Multipart;
use actix_web::{web::Path, *};
use std::{error::Error, path::PathBuf};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

use crate::{functions::*, structs::*, traits::CollectionItem, *};

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

    if !fs::try_exists(&path_buf.parent().unwrap()).await? {
        return Err(V1Error::FileNotFound.into());
    }

    let size = req
        .headers()
        .get("content-length")
        .unwrap()
        .to_str()?
        .parse::<u64>()?;

    if account
        .exceeds_limit_nosave(STORAGE_LIMITS.get().unwrap(), Some(size), None)
        .await?
    {
        return Err(V1Error::StorageFull.into());
    }

    account.stored.as_mut().unwrap().value += size;
    account.save_replace(ACCOUNTS.get().unwrap()).await?;

    let data = bytes_from_multipart(payload).await?;

    file_check_v1(&data, &path_buf)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path_buf)
        .await?;

    file.write_all(&data).await?;

    Ok(V1Response::FileItemCreated)
}
