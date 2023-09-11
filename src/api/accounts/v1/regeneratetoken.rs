use std::error::Error;

use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response, V1TokenPassword};

use crate::{functions::*, structs::*, traits::CollectionItem, *};

#[post("/regeneratetoken")]
async fn regenerate_token(post: Json<V1TokenPassword>) -> HttpResponse {
    from_res(regenerate_token_task(post).await)
}

async fn regenerate_token_task(post: Json<V1TokenPassword>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = ACCOUNTS.get().unwrap();

    let mut account = match Account::find_by_token(&post.token).await? {
        Some(account) => account,
        None => return Err(V1Error::NoSuchUser.into()),
    };

    if !account.password_matches(&post.password) {
        return Err(V1Error::PasswordIncorrect.into());
    }

    account.regeneratetoken();
    account.save_replace(accounts).await?;

    Ok(V1Response::RegenerateToken {
        token: account.token.clone(),
    })
}
