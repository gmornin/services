use std::error::Error;

use actix_files::NamedFile;
use actix_web::{get, web, HttpRequest, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response};
use tokio::fs;

use crate::{
    functions::{from_res, get_user_dir, has_dotdot, is_bson},
    structs::{ItemVisibility, Visibilities},
};

#[get("file/id/{id}/{path:.*}")]
pub async fn by_id(path: web::Path<(i64, String)>, req: HttpRequest) -> HttpResponse {
    match fetch(path, &req).await {
        Ok(ok) => ok,
        Err(e) => {
            let res: Result<V1Response, Box<dyn Error>> = Err(e);
            from_res(res)
        }
    }
}

pub async fn fetch(
    path: web::Path<(i64, String)>,
    req: &HttpRequest,
) -> Result<HttpResponse, Box<dyn Error>> {
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

    if !metadata.is_file() {
        return Err(V1Error::TypeMismatch.into());
    }

    Ok(NamedFile::open_async(path).await?.into_response(req))
}
