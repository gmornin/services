use std::{error::Error, path::PathBuf, time::UNIX_EPOCH};

use actix_files::NamedFile;
use actix_web::{routes, web, HttpRequest, HttpResponse};
use goodmorning_bindings::{
    services::v1::{V1DirItem, V1Error, V1Response},
    traits::ResTrait,
};
use tokio::fs;

use crate::{
    functions::{has_dotdot, is_bson},
    structs::{ItemVisibility, Visibilities, Visibility},
    *,
};

#[routes]
#[get("/id/{id}/{path:.*}")]
#[get("/{id}/{path:.*}")]
pub async fn by_id(path: web::Path<(String, String)>, req: HttpRequest) -> HttpResponse {
    match fetch(path, &req).await {
        Ok(ok) => ok,
        Err(e) => HttpResponse::Ok().json(V1Response::from_res(Err(e))),
    }
}

pub async fn fetch(
    path: web::Path<(String, String)>,
    req: &HttpRequest,
) -> Result<HttpResponse, Box<dyn Error>> {
    let (id, path) = path.into_inner();
    let path = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(id)
        .join(path.trim_start_matches('/'));

    if has_dotdot(&path) || is_bson(&path) {
        return Err(V1Error::PermissionDenied.into());
    }

    if !fs::try_exists(&path).await?
        || Visibilities::visibility(&path).await?.visibility == ItemVisibility::Private
    {
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

    Ok(NamedFile::open_async(path).await?.into_response(req))
}
