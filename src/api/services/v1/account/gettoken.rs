use std::error::Error;

use actix_web::{
    post,
    web::{Data, Json},
};
use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
};
use mongodb::Database;
use serde::Deserialize;

use crate::{functions::*, structs::*, *};

#[derive(Deserialize)]
struct GetToken {
    pub identifier: String,
    pub identifier_type: IdentifierType,
    pub password: String,
}

#[post("/gettoken")]
async fn get_token(post: Json<GetToken>, db: Data<Database>) -> Json<V1Response> {
    Json(V1Response::from_res(get_token_task(post, db).await))
}

async fn get_token_task(
    post: Json<GetToken>,
    db: Data<Database>,
) -> Result<V1Response, Box<dyn Error>> {
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
        None => return Err(V1Error::NoSuchUser.into()),
    };

    if !account.password_matches(&post.password) {
        return Err(V1Error::PasswordIncorrect.into());
    }

    Ok(V1Response::GetToken {
        token: account.token,
    })
}
