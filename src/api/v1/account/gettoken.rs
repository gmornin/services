use std::error::Error;

use actix_web::{
    post,
    web::{Data, Json},
};
use mongodb::Database;
use serde::Deserialize;

use crate::{api::v1::*, functions::*, structs::*, *};

#[derive(Deserialize)]
struct GetToken {
    pub identifier: String,
    pub identifier_type: IdentifierType,
    pub password: String,
}

#[post("/gettoken")]
async fn get_token(post: Json<GetToken>, db: Data<Database>) -> Json<GMResponses> {
    Json(to_res(get_token_task(post, db).await))
}

async fn get_token_task(
    post: Json<GetToken>,
    db: Data<Database>,
) -> Result<GMResponses, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(&db);

    let account = match Account::find_by_idenifier(
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

    Ok(GMResponses::GetToken {
        token: account.token,
    })
}
