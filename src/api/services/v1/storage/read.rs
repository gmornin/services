use actix_files::NamedFile;
use actix_web::{
    web::{Data, Path},
    *,
};
use mongodb::Database;
use std::error::Error;
use std::path::PathBuf;
use tokio::fs::{self, try_exists};

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::{
    services::v1::{V1DirItem, V1Error, V1Response},
    traits::ResTrait,
};

#[get("/read/{token}/{path:.*}")]
pub async fn read(
    path: Path<(String, String)>,
    req: HttpRequest,
    db: Data<Database>,
) -> HttpResponse {
    let (token, path) = path.into_inner();
    match read_task(&path, &token, &req, &db).await {
        Ok(ok) => ok,
        Err(e) => HttpResponse::Ok().json(V1Response::from_res(Err(e))),
    }
}

async fn read_task(
    path: &str,
    token: &str,
    req: &HttpRequest,
    db: &Database,
) -> Result<HttpResponse, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if !account.verified {
        return Err(V1Error::NotVerified.into());
    }

    let path_buf = PathBuf::from(USERCONTENT.as_str()).join(&account.id).join(path.trim_start_matches('/'));

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

            items.push(V1DirItem {
                name: entry.file_name().to_os_string().into_string().unwrap(),
                is_file: entry.metadata().await?.is_file(),
                visibility: items_visibilities
                    .get(entry.file_name().to_str().unwrap())
                    .overwrite_if_inherited(dir_visibilily)
                    .into(),
            });
        }

        return Ok(HttpResponse::Ok().json(items));
    }

    if !try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    Ok(NamedFile::open_async(path_buf).await?.into_response(req))
}
