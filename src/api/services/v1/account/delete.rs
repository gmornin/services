use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response, V1TokenOnly};

#[post("/delete")]
async fn delete(post: Json<V1TokenOnly>) -> HttpResponse {
    from_res(delete_task(post).await)
}

async fn delete_task(post: Json<V1TokenOnly>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(DATABASE.get().unwrap());

    let account = match Account::find_by_token(&post.token, &accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    account.delete(&accounts).await?;

    Ok(V1Response::Deleted)
}
