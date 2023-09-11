use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::*;
use tokio::fs;

#[post("/delete")]
async fn delete(post: Json<V1TokenPassword>) -> HttpResponse {
    from_res(delete_task(post).await)
}

async fn delete_task(post: Json<V1TokenPassword>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();

    let account = match Account::find_by_token(&post.token).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if !account.password_matches(&post.password) {
        return Err(V1Error::PasswordIncorrect.into());
    }

    account.delete(ACCOUNTS.get().unwrap()).await?;
    let _ = fs::remove_dir_all(get_user_dir(account.id, None)).await;

    Ok(V1Response::Deleted)
}
