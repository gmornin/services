use actix_multipart::Multipart;
use actix_web::{web::Path, *};

use std::{error::Error, path::PathBuf};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

use goodmorning_bindings::services::v1::{V1Error, V1Response};

use crate::{functions::*, structs::*, *};

#[post("/upload-overwrite/{token}/{path:.*}")]
pub async fn upload_overwrite(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> HttpResponse {
    from_res(upload_overwrite_task(payload, path, req).await)
}

async fn upload_overwrite_task(
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

    if account
        .exceeds_limit(
            STORAGE_LIMITS.get().unwrap(),
            Some(
                req.headers()
                    .get("content-length")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
            ),
            Some(match fs::metadata(&path_buf).await {
                Ok(me) => me.len(),
                Err(_) => 0,
            }),
        )
        .await
        .unwrap()
    {
        return Err(V1Error::StorageFull.into());
    }

    let data = bytes_from_multipart(payload).await?;

    file_check_v1(&data, &path_buf)?;

    if fs::try_exists(&path_buf).await? && fs::metadata(&path_buf).await?.is_dir() {
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
