use std::error::Error;

use actix_web::{
    post,
    web::{Data, Json},
};
use mongodb::Database;
use serde::Deserialize;

use crate::{api::v1::*, functions::*, structs::*, traits::CollectionItem, *};

#[derive(Deserialize)]
struct RegenerateToken {
    pub identifier: String,
    pub identifier_type: IdentifierType,
    pub password: String,
}

#[post("/regeneratetoken")]
async fn regenerate_token(post: Json<RegenerateToken>, db: Data<Database>) -> Json<GMResponses> {
    Json(to_res(regenerate_token_task(post, db).await))
}

async fn regenerate_token_task(
    post: Json<RegenerateToken>,
    db: Data<Database>,
) -> Result<GMResponses, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(&db);

    let mut account = match Account::find_by_idenifier(
        &post.identifier_type,
        post.identifier,
        &accounts,
    )
    .await?
    {
        Some(account) => account,
        None => return Err(GMError::NoSuchUser.into()),
    };

    if !account.password_matches(&post.password) {
        return Err(GMError::PasswordIncorrect.into());
    }

    account.regeneratetoken();
    account.save_replace(&accounts).await?;

    Ok(GMResponses::RegenerateToken {
        token: account.token.clone(),
    })
}
