use std::error::Error;

use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use goodmorning_bindings::services::v1::{V1All3, V1Error, V1Response};

use crate::{functions::*, structs::*, traits::CollectionItem, *};

#[post("/create")]
async fn create(post: Json<V1All3>, req: HttpRequest) -> HttpResponse {
    from_res(create_task(post, req).await)
}

async fn create_task(post: Json<V1All3>, req: HttpRequest) -> Result<V1Response, Box<dyn Error>> {
    if !ALLOW_REGISTER.get().unwrap()
        && !CREATE_WHITELIST
            .get()
            .unwrap()
            .contains(&req.connection_info().peer_addr().unwrap().to_string())
    {
        return Err(V1Error::FeatureDisabled.into());
    }

    let post = post.into_inner();
    if !Account::username_valid(&post.username) {
        return Err(V1Error::InvalidUsername.into());
    }

    if Account::find_by_username(post.username.clone())
        .await?
        .is_some()
    {
        return Err(V1Error::UsernameTaken.into());
    }

    if Account::find_by_email(&post.email).await?.is_some() {
        return Err(V1Error::EmailTaken.into());
    }

    let account = Account::new(
        post.username,
        &post.password,
        &post.email,
        DATABASE.get().unwrap(),
    )
    .await?;

    account.email_verification().await?;
    account.save_create(ACCOUNTS.get().unwrap()).await?;

    Ok(V1Response::Created {
        id: account.id,
        token: account.token,
    })
}
