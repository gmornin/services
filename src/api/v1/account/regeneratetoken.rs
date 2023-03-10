use std::error::Error;

use actix_web::{
    post,
    web::{Data, Json},
};
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::{functions::*, structs::*, traits::CollectionItem, *};

use super::{ErrorKind, Responses};

#[derive(Serialize, Deserialize)]
struct RegenerateToken {
    pub identifier: String,
    pub identifier_type: IdentifierType,
    pub password: String,
}

#[post("/regeneratetoken")]
async fn regenerate_token(post: Json<RegenerateToken>, db: Data<Database>) -> Json<Responses> {
    match regenerate_token_task(post, db).await {
        Ok(res) => Json(res),
        Err(e) => Json(Responses::Error {
            kind: match e.downcast::<ErrorKind>() {
                Ok(downcasted) => *downcasted,
                Err(e) => ErrorKind::External(e.to_string()),
            },
        }),
    }
}

async fn regenerate_token_task(
    post: Json<RegenerateToken>,
    db: Data<Database>,
) -> Result<Responses, Box<dyn Error>> {
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
        None => return Err(ErrorKind::NoSuchUser.into()),
    };

    if !account.password_matches(&post.password) {
        return Err(ErrorKind::PasswordIncorrect.into());
    }

    account.regeneratetoken();
    account.save_replace(&accounts).await?;

    Ok(Responses::RegenerateToken {
        token: account.token.clone(),
    })
}
