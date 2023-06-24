use std::error::Error;

use crate::{functions::*, structs::*, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response, V1TokenOnly};
use tokio::fs;

#[post("/reset-pfp")]
async fn reset_pfp(post: Json<V1TokenOnly>) -> HttpResponse {
    from_res(reset_pfp_task(post).await)
}

async fn reset_pfp_task(post: Json<V1TokenOnly>) -> Result<V1Response, Box<dyn Error>> {
    let accounts = ACCOUNTS.get().unwrap();

    let account = match Account::find_by_token(&post.token, accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if !account.services.contains(&GMServices::Tex) {
        return Err(V1Error::NotCreated.into());
    }

    let path = get_usersys_dir(account.id, Some("tex")).join("pfp.png");

    if !fs::try_exists(&path).await? {
        return Ok(V1Response::NothingChanged);
    }

    fs::remove_file(path).await?;

    Ok(V1Response::PfpReset)
}
