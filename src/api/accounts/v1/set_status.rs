use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response, V1SetStatus};

#[post("/set-status")]
async fn set_status(post: Json<V1SetStatus>) -> HttpResponse {
    from_res(set_status_task(post).await)
}

async fn set_status_task(post: Json<V1SetStatus>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();

    let mut account = match Account::find_by_token(&post.token).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if post.new.len() > 128 {
        return Err(V1Error::ExceedsMaximumLength.into());
    }

    account.status = post.new;
    account.save_replace(ACCOUNTS.get().unwrap()).await?;

    Ok(V1Response::ProfileUpdated)
}
