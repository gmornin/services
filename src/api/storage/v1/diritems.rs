use actix_web::{web::Path, *};
use std::error::Error;

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::services::v1::{V1Error, V1Response};

#[get("/diritems/{token}/{path:.*}")]
pub async fn diritems(path: Path<(String, String)>) -> HttpResponse {
    from_res(diritems_task(path).await)
}

async fn diritems_task(path: Path<(String, String)>) -> Result<V1Response, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let accounts = ACCOUNTS.get().unwrap();
    let account = match Account::find_by_token(&token, accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if !account.verified {
        return Err(V1Error::NotVerified.into());
    }

    Ok(V1Response::DirContent {
        content: dir_items(account.id, std::path::Path::new(&path), true, false).await?,
    })
}
