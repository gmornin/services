use std::error::Error;

use actix_files::NamedFile;
use actix_web::{
    get,
    http::{self, header::ContentDisposition},
    web::{self, Query},
    HttpRequest, HttpResponse,
};
use goodmorning_bindings::services::v1::{V1Error, V1Response};
use serde::Deserialize;
use tokio::fs;

use crate::{
    functions::{from_res, get_user_dir, has_dotdot, is_bson},
    structs::{ItemVisibility, Visibilities},
};

#[derive(Deserialize)]
struct DisplayType {
    #[serde(rename = "display")]
    pub display: Option<String>,
}

#[get("file/id/{id}/{path:.*}")]
pub async fn by_id(
    path: web::Path<(i64, String)>,
    req: HttpRequest,
    query: Query<DisplayType>,
) -> HttpResponse {
    match fetch(path, &req, query).await {
        Ok(ok) => ok,
        Err(e) => from_res::<V1Response>(Err(e)),
    }
}

async fn fetch(
    path: web::Path<(i64, String)>,
    req: &HttpRequest,
    query: Query<DisplayType>,
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

    Ok(match query.display.as_deref().unwrap_or_default() {
        "inline" => {
            NamedFile::open_async(path)
                .await?
                .set_content_disposition(ContentDisposition {
                    disposition: http::header::DispositionType::Inline,
                    parameters: Vec::new(),
                })
        }
        "text" => NamedFile::open_async(path)
            .await?
            .set_content_type(mime::TEXT_PLAIN)
            .set_content_disposition(ContentDisposition {
                disposition: http::header::DispositionType::Inline,
                parameters: Vec::new(),
            }),
        _ => NamedFile::open_async(path).await?,
    }
    .into_response(req))
}
