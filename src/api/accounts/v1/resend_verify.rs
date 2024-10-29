use std::error::Error;

use crate::{
    functions::*, structs::*, traits::CollectionItem, ACCOUNTS, EMAIL_VERIFICATION_COOLDOWN,
    VERIFICATION,
};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response, V1TokenOnly};

#[post("/resend-verify")]
pub async fn resend_verify(post: Json<V1TokenOnly>) -> HttpResponse {
    from_res(resend_verify_task(post).await)
}

async fn resend_verify_task(post: Json<V1TokenOnly>) -> Result<V1Response, Box<dyn Error>> {
    if !VERIFICATION.get().unwrap() {
        return Err(V1Error::FeatureDisabled.into());
    }

    let mut account = Account::v1_get_by_token(&post.token).await?;

    if account.verified {
        return Ok(V1Response::NothingChanged);
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

    account.email_verification().await?;
    account.last_verify = now;

    account.save_replace(ACCOUNTS.get().unwrap()).await?;

    Ok(V1Response::VerificationSent)
}
