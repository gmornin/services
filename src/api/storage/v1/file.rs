use actix_files::NamedFile;
use actix_web::{
    http::header::ContentDisposition,
    web::{Path, Query},
    *,
};
use serde::Deserialize;
use std::{error::Error, path::PathBuf};
use tokio::fs::{self, try_exists};

use crate::{functions::*, structs::*};

use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
};

#[derive(Deserialize)]
struct DisplayType {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[get("/file/{token}/{path:.*}")]
pub async fn file(
    path: Path<(String, String)>,
    req: HttpRequest,
    query: Query<DisplayType>,
) -> HttpResponse {
    match file_task(path, &req, query).await {
        Ok(ok) => ok,
        Err(e) => HttpResponse::NotFound().json(V1Response::from_res(Err(e))),
    }
}

async fn file_task(
    path: Path<(String, String)>,
    req: &HttpRequest,
    query: Query<DisplayType>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    let user_path = PathBuf::from(path.trim_start_matches('/'));

    if has_dotdot(&user_path) || is_bson(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }

    let path_buf = get_user_dir(account.id, None).join(user_path);

    if !try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if fs::metadata(&path_buf).await?.is_dir() {
        return Err(V1Error::TypeMismatch.into());
    }

    Ok(match query.r#type.as_deref().unwrap_or_default() {
        "inline" => NamedFile::open_async(path_buf)
            .await?
            .set_content_disposition(ContentDisposition {
                disposition: http::header::DispositionType::Inline,
                parameters: Vec::new(),
            }),
        _ => NamedFile::open_async(path_buf).await?,
    }
    .into_response(req))
}
