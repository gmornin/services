use std::error::Error;

use actix_web::{post, web::Json};
use goodmorning_bindings::{
    services::v1::{V1Error, V1PasswordId, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*, *};

#[post("/login")]
async fn login(post: Json<V1PasswordId>) -> Json<V1Response> {
    Json(V1Response::from_res(login_task(post).await))
}

async fn login_task(post: Json<V1PasswordId>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(DATABASE.get().unwrap());

    let account =
        match Account::find_by_idenifier(&post.identifier_type.into(), post.identifier, &accounts)
            .await?
        {
            Some(account) => account,
            None => return Err(V1Error::NoSuchUser.into()),
        };

    if !account.password_matches(&post.password) {
        return Err(V1Error::PasswordIncorrect.into());
    }

    Ok(V1Response::Login {
        token: account.token,
        id: account.id,
    })
}
