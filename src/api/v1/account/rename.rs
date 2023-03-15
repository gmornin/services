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
struct RenameAccount {
    pub token: String,
    pub new: String,
}

#[post("/rename")]
async fn rename(post: Json<RenameAccount>, db: Data<Database>) -> Json<Responses> {
    match rename_task(post, db).await {
        Ok(res) => Json(res), 
        Err(e) => Json(Responses::Error {
            kind: match e.downcast::<ErrorKind>() {
                Ok(downcasted) => *downcasted,
                Err(e) => ErrorKind::External(e.to_string()),
            },
        }),
    }
}

async fn rename_task(
    post: Json<RenameAccount>,
    db: Data<Database>,
) -> Result<Responses, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(&db);

    let mut account = match Account::find_by_token(&post.token, &accounts)
    .await?
    {
        Some(account) => account,
        None => return Err(ErrorKind::InvalidToken.into()),
    };

    if Account::find_by_username(post.new.clone(), &accounts).await?.is_some() {
        return Err(ErrorKind::UsernameTaken.into());
    }

    account.username = post.new;
    account.save_replace(&accounts).await?;

    Ok(Responses::Deleted)
}
