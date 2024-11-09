use std::error::Error;

use crate::{functions::*, structs::*, *};
use actix_web::{post, web::Json, HttpResponse};
use futures_util::StreamExt;
use goodmorning_bindings::services::v1::{
    AccessType, V1Error, V1Response, V1SimpleUser, V1TokenAccessTypeOptionIdentifier,
};
use mongodb::bson::doc;

#[post("/accessto")]
pub async fn accessto(post: Json<V1TokenAccessTypeOptionIdentifier>) -> HttpResponse {
    from_res(accessto_task(post).await)
}

async fn accessto_task(
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
            if account
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

    let mut cursor = ACCOUNTS
        .get()
        .unwrap()
        .find(doc! { format!("access.{}", post.access_type.as_str()): account.id })
        .await?;

    while let Some(user) = cursor.next().await {
        let user = user?;

        users.push(V1SimpleUser {
            id: user.id,
            username: user.username,
        });
    }

    Ok(V1Response::AllowedAccess { users })
}
