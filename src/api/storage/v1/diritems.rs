use actix_web::{web::Path, *};
use std::error::Error;

use crate::{functions::*, structs::*};

use goodmorning_bindings::services::v1::V1Response;

#[get("/diritems/{token}/{path:.*}")]
pub async fn diritems(path: Path<(String, String)>) -> HttpResponse {
    from_res(diritems_task(path).await)
}

async fn diritems_task(path: Path<(String, String)>) -> Result<V1Response, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    Ok(V1Response::DirContent {
        content: dir_items(account.id, std::path::Path::new(&path), true, false).await?,
    })
}
