use std::error::Error;

use crate::{
    functions::*, structs::*, traits::CollectionItem, ACCOUNTS, EMAIL_VERIFICATION_COOLDOWN,
    VERIFICATION,
};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1ChangeEmail, V1Error, V1Response};

#[post("/change-email")]
async fn change_email(post: Json<V1ChangeEmail>) -> HttpResponse {
    from_res(change_email_task(post).await)
}

async fn change_email_task(post: Json<V1ChangeEmail>) -> Result<V1Response, Box<dyn Error>> {
    let mut account = Account::v1_get_by_token(&post.token).await?;

    if !account.password_matches(&post.password) {
        return Err(V1Error::PasswordIncorrect.into());
    }

    let now = chrono::Utc::now().timestamp() as u64;
    let cooldown = EMAIL_VERIFICATION_COOLDOWN.get().unwrap();
    let diff = now - account.last_verify;
    if &diff < cooldown {
        return Err(V1Error::Cooldown {
            remaining: *cooldown - diff,
        }
        .into());
    }

    let new = post.new.to_lowercase();
    if account.email == new {
        return Ok(V1Response::NothingChanged);
    }

    if Account::find_by_email(&post.new).await?.is_some() {
        return Err(V1Error::EmailTaken.into());
    }

    if !Account::is_email_valid(&new) {
        return Err(V1Error::Blacklisted.into());
    }

    account.verified = false;
    account.email = new;

    account.email_verification().await?;
    account.last_verify = now;

    account.save_replace(ACCOUNTS.get().unwrap()).await?;

    Ok(V1Response::EmailChanged {
        verify: *VERIFICATION.get().unwrap(),
    })
}
