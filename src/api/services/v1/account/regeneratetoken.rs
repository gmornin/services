use std::error::Error;

use actix_web::{
    post,
    web::{Data, Json},
};
use goodmorning_bindings::{
    services::v1::{V1Error, V1PasswordId, V1Response},
    traits::ResTrait,
};
use mongodb::Database;

use crate::{functions::*, structs::*, traits::CollectionItem};

#[post("/regeneratetoken")]
async fn regenerate_token(post: Json<V1PasswordId>, db: Data<Database>) -> Json<V1Response> {
    Json(V1Response::from_res(regenerate_token_task(post, db).await))
}

async fn regenerate_token_task(
    post: Json<V1PasswordId>,
    db: Data<Database>,
) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(&db);

    let mut account =
        match Account::find_by_idenifier(&post.identifier_type.into(), post.identifier, &accounts)
            .await?
        {
            Some(account) => account,
            None => return Err(V1Error::NoSuchUser.into()),
        };

    if !account.password_matches(&post.password) {
        return Err(V1Error::PasswordIncorrect.into());
    }

    account.regeneratetoken();
    account.save_replace(&accounts).await?;

    Ok(V1Response::RegenerateToken {
        token: account.token.clone(),
    })
}
