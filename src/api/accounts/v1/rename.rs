use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1RenameAccount, V1Response};

#[post("/rename")]
pub async fn rename(post: Json<V1RenameAccount>) -> HttpResponse {
    from_res(rename_task(post).await)
}

async fn rename_task(post: Json<V1RenameAccount>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();

    if !Account::username_valid(&post.new) {
        return Err(V1Error::InvalidUsername.into());
    }

    let mut account = match Account::find_by_token(&post.token).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if post.new.eq_ignore_ascii_case(&account.username) {
        account.username = post.new;
        account.save_replace(ACCOUNTS.get().unwrap()).await?;
        return Ok(V1Response::Renamed);
    }

    if Account::find_by_username(post.new.clone()).await?.is_some() {
        return Err(V1Error::UsernameTaken.into());
    }

    account.username = post.new;
    account.save_replace(ACCOUNTS.get().unwrap()).await?;

    Ok(V1Response::Renamed)
}
