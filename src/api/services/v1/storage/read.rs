use actix_files::NamedFile;
use actix_web::{
    web::{Data, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use tokio::fs::{self, try_exists};

use crate::{
    api::services::v1::*,
    functions::*,
    structs::*,
};

#[derive(Deserialize)]
struct StaticPath {
    path: String,
    token: String,
}

#[get("/read/{path:.*}")]
pub async fn read(path: Path<StaticPath>, req: HttpRequest, db: Data<Database>) -> HttpResponse {
    match read_task(&path.path, &path.token, &req, &db).await {
        Ok(ok) => ok,
        Err(e) => HttpResponse::Ok().json(to_res::<GMResponses>(Err(e))),
    }
}

async fn read_task(
    path: &str,
    token: &str,
    req: &HttpRequest,
    db: &Database,
) -> Result<HttpResponse, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await.unwrap() {
        Some(account) => account,
        None => return Err(GMError::InvalidToken.into()),
    };

    let path_buf = PathBuf::from(format!("usercontent/{}/{}", account.id, path));

    if !try_exists(&path_buf).await.unwrap() {
        return Err(GMError::FileNotFound.into());
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

            items.push(DirItem {
                name: entry.file_name().to_os_string().into_string().unwrap(),
                is_file: entry.metadata().await?.is_file(),
                visibility: items_visibilities.get(entry.file_name().to_str().unwrap()).overwrite_if_inherited(dir_visibilily)
            });
        }

        return Ok(HttpResponse::Ok().json(items));
    }

    if !try_exists(&path_buf).await.unwrap() {
        return Err(GMError::FileNotFound.into());
    }

    Ok(NamedFile::open_async(path_buf).await?.into_response(req))
}
