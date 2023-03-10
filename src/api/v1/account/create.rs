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
struct CreateAccount {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/create")]
async fn create(post: Json<CreateAccount>, db: Data<Database>) -> Json<Responses> {
    match create_task(post, db).await {
        Ok(res) => Json(res),
        Err(e) => Json(Responses::Error {
            kind: match e.downcast::<ErrorKind>() {
                Ok(downcasted) => *downcasted,
                Err(e) => ErrorKind::External(e.to_string()),
            },
        }),
    }
}

async fn create_task(
    post: Json<CreateAccount>,
    db: Data<Database>,
) -> Result<Responses, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(&db);
    let triggers = get_triggers(&db);

    if Account::find_by_username(post.username.clone(), &accounts)
        .await?
        .is_some()
    {
        return Err(ErrorKind::UsernameTaken.into());
    }

    if Account::find_by_email(&post.email, &accounts)
        .await?
        .is_some()
    {
        return Err(ErrorKind::EmailTaken.into());
    }

    let account = Account::new(post.username, &post.password, &post.email);

    let trigger = Trigger::new_from_action(
        Box::new(account.email_verification()),
        &EMAIL_VERIFICATION_DURATION,
    );
    trigger.init(&db).await?;
    trigger.save_create(&triggers).await?;

    account.save_create(&accounts).await?;

    Ok(Responses::Created { id: account.id })
}
