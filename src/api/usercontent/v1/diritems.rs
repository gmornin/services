use std::{error::Error, time::UNIX_EPOCH};

use actix_web::{get, web, HttpResponse};
use goodmorning_bindings::services::v1::{V1DirItem, V1Error, V1Response};
use tokio::fs;

use crate::{
    functions::{from_res, get_user_dir, has_dotdot, is_bson},
    structs::{ItemVisibility, Visibilities, Visibility},
};

#[get("/diritems/id/{id}/{path:.*}")]
pub async fn by_id(path: web::Path<(i64, String)>) -> HttpResponse {
    from_res(task(path).await)
}

pub async fn task(path: web::Path<(i64, String)>) -> Result<V1Response, Box<dyn Error>> {
    let (id, path) = path.into_inner();
    let path = get_user_dir(id, None).join(path.trim_start_matches('/'));

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

    if !metadata.is_dir() {
        return Err(V1Error::TypeMismatch.into());
    }

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

    Ok(V1Response::DirContent { content: items })
}
