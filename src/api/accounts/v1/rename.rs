use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1RenameAccount, V1Response};

#[post("/rename")]
async fn rename(post: Json<V1RenameAccount>) -> HttpResponse {
    from_res(rename_task(post).await)
}

async fn rename_task(post: Json<V1RenameAccount>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(DATABASE.get().unwrap());

    let re = regex::Regex::new(r"^[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)*$").unwrap();
    if !re.is_match(&post.new) || post.new.len() > 32 || post.new.len() < 3 {
        return Err(V1Error::InvalidUsername.into());
    }

    let mut account = match Account::find_by_token(&post.token, &accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if Account::find_by_username(post.new.clone(), &accounts)
        .await?
        .is_some()
    {
        return Err(V1Error::UsernameTaken.into());
    }

    account.username = post.new;
    account.save_replace(&accounts).await?;

    Ok(V1Response::Renamed)
}
