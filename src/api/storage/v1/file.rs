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

use goodmorning_bindings::services::v1::{V1Error, V1Response};

#[derive(Deserialize)]
struct DisplayType {
    #[serde(rename = "display")]
    pub display: Option<String>,
}

#[get("/file/{token}/{path:.*}")]
pub async fn file(
    path: Path<(String, String)>,
    req: HttpRequest,
    query: Query<DisplayType>,
) -> HttpResponse {
    match file_task(path, &req, query).await {
        Ok(ok) => ok,
        Err(e) => from_res::<V1Response>(Err(e)),
    }
}

async fn file_task(
    path: Path<(String, String)>,
    req: &HttpRequest,
    query: Query<DisplayType>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let mut account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    let mut user_path = PathBuf::from(path.trim_start_matches('/'));

    if let [_, "Shared", user, ..] = user_path
        .iter()
        .map(|s| s.to_str().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
    {
        account = if let Some(account) = Account::find_by_username(user.to_string()).await? {
            account.v1_restrict_verified()?
        } else {
            return Err(V1Error::FileNotFound.into());
        };
        user_path = [user_path.iter().next().unwrap()]
            .into_iter()
            .chain(user_path.iter().skip(3))
            .collect();
    }

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

    Ok(match query.display.as_deref().unwrap_or_default() {
        "inline" => NamedFile::open_async(path_buf)
            .await?
            .set_content_disposition(ContentDisposition {
                disposition: http::header::DispositionType::Inline,
                parameters: Vec::new(),
            }),
        "text" => NamedFile::open_async(path_buf)
            .await?
            .set_content_type(mime::TEXT_PLAIN)
            .set_content_disposition(ContentDisposition {
                disposition: http::header::DispositionType::Inline,
                parameters: Vec::new(),
            }),
        _ => NamedFile::open_async(path_buf).await?,
    }
    .into_response(req))
}
