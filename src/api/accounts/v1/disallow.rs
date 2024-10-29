use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Access, V1Error, V1Response};

#[post("/disallow")]
pub async fn disallow(post: Json<V1Access>) -> HttpResponse {
    from_res(disallow_task(post).await)
}

async fn disallow_task(post: Json<V1Access>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();

    let mut account = match Account::find_by_token(&post.token).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    }
    .v1_restrict_verified()?;

    let target =
        match Account::find_by_idenifier(&post.identifier_type.into(), post.identifier).await? {
            Some(account) => account,
            None => return Err(V1Error::NoSuchUser.into()),
        };

    let entry = account.access.entry(post.r#type).or_default();
    if !entry.contains(&target.id) {
        return Ok(V1Response::NothingChanged);
    }

    entry.remove(&target.id);
    account.save_replace(ACCOUNTS.get().unwrap()).await?;

    Ok(V1Response::Disallowed)
}
