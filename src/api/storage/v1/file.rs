use actix_files::NamedFile;
use actix_web::{web::Path, *};
use std::error::Error;
use tokio::fs::{self, try_exists};

use crate::{functions::*, structs::*};

use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
};

#[get("/file/{token}/{path:.*}")]
pub async fn file(path: Path<(String, String)>, req: HttpRequest) -> HttpResponse {
    match file_task(path, &req).await {
        Ok(ok) => ok,
        Err(e) => HttpResponse::NotFound().json(V1Response::from_res(Err(e))),
    }
}

async fn file_task(
    path: Path<(String, String)>,
    req: &HttpRequest,
) -> Result<HttpResponse, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    let path_buf = get_user_dir(account.id, None).join(path.trim_start_matches('/'));

    if has_dotdot(&path_buf) || is_bson(&path_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if !try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if fs::metadata(&path_buf).await?.is_dir() {
        return Err(V1Error::TypeMismatch.into());
    }

    Ok(NamedFile::open_async(path_buf).await?.into_response(req))
}
