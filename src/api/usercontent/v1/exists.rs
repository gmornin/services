use std::error::Error;

use actix_web::{get, web, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response};
use tokio::fs;

use crate::{
    functions::{from_res, get_user_dir, has_dotdot, is_bson},
    structs::{ItemVisibility, Visibilities},
};

#[get("exists/id/{id}/{path:.*}")]
pub async fn by_id(path: web::Path<(i64, String)>) -> HttpResponse {
    from_res(exists(path).await)
}

pub async fn exists(path: web::Path<(i64, String)>) -> Result<V1Response, Box<dyn Error>> {
    let (id, path) = path.into_inner();
    let path = get_user_dir(id, None).join(path.trim_start_matches('/'));

    if has_dotdot(&path) || is_bson(&path) {
        return Err(V1Error::PermissionDenied.into());
    }

    if Visibilities::visibility(&path).await?.visibility == ItemVisibility::Private {
        return Ok(V1Response::Exists { value: false });
    }
    Ok(V1Response::Exists {
        value: fs::try_exists(&path).await?,
    })
}
