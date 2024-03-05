use actix_web::{web::Path, *};
use std::error::Error;
use tokio::fs;

use crate::{functions::*, structs::*};

use goodmorning_bindings::services::v1::{V1Error, V1Response};

#[get("/tree/{token}/{path:.*}")]
pub async fn tree(path: Path<(String, String)>) -> HttpResponse {
    from_res(tree_task(path).await)
}

async fn tree_task(path: Path<(String, String)>) -> Result<V1Response, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    let src = get_user_dir(account.id, None).join(path);

    if !fs::try_exists(&src.parent().unwrap()).await? {
        return Err(V1Error::FileNotFound.into());
    }

    Ok(V1Response::Tree {
        content: dirtree(&src, true, Visibilities::visibility(&src).await?).await?,
    })
}
