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
struct DeleteAccount {
    pub token: String,
}

#[post("/delete")]
async fn delete(post: Json<DeleteAccount>, db: Data<Database>) -> Json<Responses> {
    match delete_task(post, db).await {
        Ok(res) => Json(res),
        Err(e) => Json(Responses::Error {
            kind: match e.downcast::<ErrorKind>() {
                Ok(downcasted) => *downcasted,
                Err(e) => ErrorKind::External(e.to_string()),
            },
        }),
    }
}

async fn delete_task(
    post: Json<DeleteAccount>,
    db: Data<Database>,
) -> Result<Responses, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(&db);

    let account = match Account::find_by_token(&post.token, &accounts)
    .await?
    {
        Some(account) => account,
        None => return Err(ErrorKind::InvalidToken.into()),
    };

    account.delete(&accounts).await?;

    Ok(Responses::Deleted)
}
