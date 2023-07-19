use std::error::Error;

use crate::{functions::*, structs::*, traits::CollectionItem, ACCOUNTS};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1ChangePassword, V1Error, V1Response};

#[post("/change-password")]
async fn changepw(post: Json<V1ChangePassword>) -> HttpResponse {
    from_res(changepw_task(post).await)
}

async fn changepw_task(post: Json<V1ChangePassword>) -> Result<V1Response, Box<dyn Error>> {
    let mut account = Account::v1_get_by_token(&post.token).await?;

    if !account.password_matches(&post.old) {
        return Err(V1Error::PasswordIncorrect.into());
    }

    account.change_password(&post.new);
    account.save_replace(ACCOUNTS.get().unwrap()).await?;

    Ok(V1Response::PasswordChanged)
}
