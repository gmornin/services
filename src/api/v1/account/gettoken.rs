use std::error::Error;

use actix_web::{
    post,
    web::{Data, Json},
};
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::{functions::*, structs::*, *};

use super::{ErrorKind, Responses};

#[derive(Serialize, Deserialize)]
struct GetToken {
    pub identifier: String,
    pub identifier_type: IdentifierType,
    pub password: String,
}

#[post("/gettoken")]
async fn get_token(post: Json<GetToken>, db: Data<Database>) -> Json<Responses> {
    match get_token_task(post, db).await {
        Ok(res) => Json(res),
        Err(e) => Json(Responses::Error {
            kind: match e.downcast::<ErrorKind>() {
                Ok(downcasted) => *downcasted,
                Err(e) => ErrorKind::External(e.to_string()),
            },
        }),
    }
}

async fn get_token_task(
    post: Json<GetToken>,
    db: Data<Database>,
) -> Result<Responses, Box<dyn Error>> {
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
        None => return Err(ErrorKind::NoSuchUser.into()),
    };

    if !account.password_matches(&post.password) {
        return Err(ErrorKind::PasswordIncorrect.into());
    }

    Ok(Responses::GetToken {
        token: account.token.clone(),
    })
}
