use std::{error::Error, path::PathBuf};

use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse, HttpRequest};
use goodmorning_bindings::{services::v1::{V1Error, V1DirItem, V1Response}, traits::ResTrait};
use tokio::fs;

use crate::{structs::{Visibilities, Visibility, ItemVisibility}, USERCONTENT, functions::{has_dotdot, is_bson}};

#[get("/id/{id}/{path:.*}")]
pub async fn by_id(path: web::Path<(String, String)>, req: HttpRequest) -> HttpResponse {
    let (id, path) = path.into_inner();
    match fetch(&id, &path, &req).await {
        Ok(ok) => ok,
        Err(e) => HttpResponse::Ok().json(V1Response::from_res(Err(e))),
    }
}

pub async fn fetch(id: &str, path: &str, req: &HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let path = PathBuf::from(USERCONTENT.as_str()).join(id).join(path.trim_start_matches('/'));

    if has_dotdot(&path) || is_bson(&path) {
        return Err(V1Error::PermissionDenied.into());
    }

    if !fs::try_exists(&path).await? || Visibilities::visibility(&path).await?.visibility == ItemVisibility::Private {
        return Err(V1Error::FileNotFound.into());
    }
    let metadata = fs::metadata(&path).await?;

    // if metadata.is_symlink() {
    //     return Err(V1Error::FileNotFound.into());
    // }

    if metadata.is_dir() {
        let mut items = Vec::new();
        let mut dir_content = fs::read_dir(&path).await?;
        let dir_visibilily = Visibilities::visibility(&path).await?;
        let items_visibilities = Visibilities::read_dir(&path).await?;

        while let Some(entry) = dir_content.next_entry().await? {
            let visibility: Visibility = items_visibilities
                    .get(entry.file_name().to_str().unwrap())
                    .overwrite_if_inherited(dir_visibilily);

            if is_bson(&entry.path()) || visibility.visibility != ItemVisibility::Public {
                continue;
            }

            items.push(V1DirItem {
                name: entry.file_name().to_os_string().into_string().unwrap(),
                is_file: entry.metadata().await?.is_file(),
                visibility: visibility.into(),
            });
        }

        return Ok(HttpResponse::Ok().json(items));
    }

    Ok(NamedFile::open_async(path).await?.into_response(req))
}
