use std::error::Error;

use actix_web::{
    post,
    web::{Data, Json},
};
use mongodb::Database;
use serde::Deserialize;

use crate::{api::v1::*, functions::*, structs::*, traits::CollectionItem, *};

#[derive(Deserialize)]
struct RenameAccount {
    pub token: String,
    pub new: String,
}

#[post("/rename")]
async fn rename(post: Json<RenameAccount>, db: Data<Database>) -> Json<GMResponses> {
    Json(to_res(rename_task(post, db).await))
}

async fn rename_task(
    post: Json<RenameAccount>,
    db: Data<Database>,
) -> Result<GMResponses, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(&db);

    let mut account = match Account::find_by_token(&post.token, &accounts).await? {
        Some(account) => account,
        None => return Err(GMError::InvalidToken.into()),
    };

    if Account::find_by_username(post.new.clone(), &accounts)
        .await?
        .is_some()
    {
        return Err(GMError::UsernameTaken.into());
    }

    account.username = post.new;
    account.save_replace(&accounts).await?;

    Ok(GMResponses::Deleted)
}
