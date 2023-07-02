use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response, V1TokenOnly};
use tokio::fs;

#[post("/create")]
async fn create(post: Json<V1TokenOnly>) -> HttpResponse {
    from_res(create_task(post).await)
}

async fn create_task(post: Json<V1TokenOnly>) -> Result<V1Response, Box<dyn Error>> {
    let accounts = ACCOUNTS.get().unwrap();

    let mut account = match Account::find_by_token(&post.token, accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if account.services.contains(&GMServices::Tex) {
        return Err(V1Error::AlreadyCreated.into());
    }

    let path = get_usersys_dir(account.id, Some(GMServices::Tex));
    fs::create_dir_all(&path).await?;

    account.services.push(GMServices::Tex);
    account.save_replace(accounts).await?;

    Ok(V1Response::ServiceCreated)
}
