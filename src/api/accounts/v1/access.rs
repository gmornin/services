use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{
    AccessType, V1Error, V1Response, V1SimpleUser, V1TokenAccessTypeOptionIdentifier,
};

#[post("/access")]
pub async fn access(post: Json<V1TokenAccessTypeOptionIdentifier>) -> HttpResponse {
    from_res(access_task(post).await)
}

async fn access_task(
    post: Json<V1TokenAccessTypeOptionIdentifier>,
) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();

    let mut account = match Account::find_by_token(&post.token).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if post.identifier.is_some()
        && post.identifier_type.is_some()
        && !account.matches_identifier(
            post.identifier.as_ref().unwrap(),
            post.identifier_type.unwrap().into(),
        )
    {
        if let Some(target) = Account::find_by_idenifier(
            &post.identifier_type.unwrap().into(),
            post.identifier.unwrap(),
        )
        .await?
        {
            if target
                .access
                .get(AccessType::Access.as_str())
                .is_some_and(|map| map.contains(&account.id))
            {
                account = target;
            } else {
                return Err(V1Error::PermissionDenied.into());
            }
        } else {
            return Err(V1Error::PermissionDenied.into());
        }
    }

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
