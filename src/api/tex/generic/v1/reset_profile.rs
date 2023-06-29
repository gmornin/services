use std::error::Error;

use crate::{functions::*, structs::*, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response, V1TokenOnly};

#[post("/reset-profile")]
async fn reset_pf(post: Json<V1TokenOnly>) -> HttpResponse {
    from_res(reset_profile_task(post).await)
}

async fn reset_profile_task(post: Json<V1TokenOnly>) -> Result<V1Response, Box<dyn Error>> {
    let accounts = ACCOUNTS.get().unwrap();

    let account = match Account::find_by_token(&post.token, accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if !account.services.contains(&GMServices::Tex) {
        return Err(V1Error::NotCreated.into());
    }

    reset_profile(account.id, "tex").await?;

    Ok(V1Response::ProfileUpdated)
}
