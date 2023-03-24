use std::error::Error;

use actix_web::{
    post,
    web::{Data, Json},
};
use mongodb::Database;
use serde::Deserialize;

use crate::{functions::*, structs::*, traits::CollectionItem, *, api::v1::*};


#[derive(Deserialize)]
struct DeleteAccount {
    pub token: String,
}

#[post("/delete")]
async fn delete(post: Json<DeleteAccount>, db: Data<Database>) -> Json<GMResponses> {
    Json(to_res(delete_task(post, db).await))
}

async fn delete_task(
    post: Json<DeleteAccount>,
    db: Data<Database>,
) -> Result<GMResponses, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(&db);

    let account = match Account::find_by_token(&post.token, &accounts).await? {
        Some(account) => account,
        None => return Err(GMError::InvalidToken.into()),
    };

    account.delete(&accounts).await?;

    Ok(GMResponses::Deleted)
}
