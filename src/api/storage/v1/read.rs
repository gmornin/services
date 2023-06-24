use actix_files::NamedFile;
use actix_web::{web::Path, *};

use std::{error::Error, time::UNIX_EPOCH};
use tokio::fs::{self, try_exists};

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::{
    services::v1::{V1DirItem, V1Error, V1Response},
    traits::ResTrait,
};

#[get("/read/{token}/{path:.*}")]
pub async fn read(path: Path<(String, String)>, req: HttpRequest) -> HttpResponse {
    match read_task(path, &req).await {
        Ok(ok) => ok,
        Err(e) => HttpResponse::Ok().json(V1Response::from_res(Err(e))),
    }
}

async fn read_task(
    path: Path<(String, String)>,
    req: &HttpRequest,
) -> Result<HttpResponse, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let accounts = ACCOUNTS.get().unwrap();
    let account = match Account::find_by_token(&token, accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if !account.verified {
        return Err(V1Error::NotVerified.into());
    }

    let path_buf = get_user_dir(account.id, None).join(path.trim_start_matches('/'));

    if has_dotdot(&path_buf) || is_bson(&path_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if !try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if fs::metadata(&path_buf).await?.is_dir() {
        let mut items = Vec::new();
        let mut dir_content = fs::read_dir(&path_buf).await?;
        let dir_visibilily = Visibilities::visibility(&path_buf).await?;
        let items_visibilities = Visibilities::read_dir(&path_buf).await?;

        while let Some(entry) = dir_content.next_entry().await? {
            if is_bson(&entry.path()) {
                continue;
            }

            let metadata = entry.metadata().await?;

            items.push(V1DirItem {
                name: entry.file_name().to_os_string().into_string().unwrap(),
                is_file: metadata.is_file(),
                visibility: items_visibilities
                    .get(entry.file_name().to_str().unwrap())
                    .overwrite_if_inherited(dir_visibilily)
                    .into(),
                last_modified: metadata
                    .modified()?
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                size: metadata.len(),
            });
        }

        return Ok(HttpResponse::Ok().json(items));
    }

    Ok(NamedFile::open_async(path_buf).await?.into_response(req))
}
