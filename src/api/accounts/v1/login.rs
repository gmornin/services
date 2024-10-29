use std::error::Error;

use actix_web::{http::StatusCode, post, web::Json, HttpResponse, HttpResponseBuilder};
use goodmorning_bindings::{
    services::v1::{V1Error, V1PasswordId, V1Response},
    traits::ResTrait,
};

use crate::structs::*;

#[post("/login")]
pub async fn login(post: Json<V1PasswordId>) -> HttpResponse {
    let res = V1Response::from_res(login_task(post).await);
    HttpResponseBuilder::new(StatusCode::from_u16(res.status_code()).unwrap()).json(res)
}

async fn login_task(post: Json<V1PasswordId>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();

    let account =
        match Account::find_by_idenifier(&post.identifier_type.into(), post.identifier).await? {
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
