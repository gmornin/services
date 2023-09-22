use std::{error::Error, path::PathBuf};

use actix_web::{get, web, HttpResponse};
use goodmorning_bindings::services::v1::V1Response;

use crate::functions::{dir_items, from_res};

#[get("/diritems/id/{id}/{path:.*}")]
pub async fn by_id(path: web::Path<(i64, PathBuf)>) -> HttpResponse {
    from_res(task(path).await)
}

pub async fn task(path: web::Path<(i64, PathBuf)>) -> Result<V1Response, Box<dyn Error>> {
    let (id, path) = path.into_inner();

    Ok(V1Response::DirContent {
        content: dir_items(id, &path, false, true).await?,
    })
}
