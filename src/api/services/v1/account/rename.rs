use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem};
use actix_web::{
    post,
    web::{Data, Json},
};
use goodmorning_bindings::{
    services::v1::{V1Error, V1RenameAccount, V1Response},
    traits::ResTrait,
};
use mongodb::Database;

#[post("/rename")]
async fn rename(post: Json<V1RenameAccount>, db: Data<Database>) -> Json<V1Response> {
    Json(V1Response::from_res(rename_task(post, db).await))
}

async fn rename_task(
    post: Json<V1RenameAccount>,
    db: Data<Database>,
) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(&db);

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
