use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response, V1SimpleUser, V1TokenAccessType};

#[post("/access")]
pub async fn access(post: Json<V1TokenAccessType>) -> HttpResponse {
    from_res(access_task(post).await)
}

async fn access_task(post: Json<V1TokenAccessType>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();

    let mut account = match Account::find_by_token(&post.token).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    }
    .v1_restrict_verified()?;

    let mut users = Vec::new();

    for id in account
        .access
        .entry(post.access_type.as_str().to_string())
        .or_default()
        .iter()
    {
        users.push(V1SimpleUser {
            id: *id,
            username: Account::find_by_id(*id, ACCOUNTS.get().unwrap())
                .await?
                .unwrap()
                .username,
        });
    }

    Ok(V1Response::Access { users })
}
