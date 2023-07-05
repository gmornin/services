use std::error::Error;

use crate::{functions::*, structs::*};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Response, V1TokenOnly};

#[post("/reset-profile")]
async fn reset_pf(post: Json<V1TokenOnly>) -> HttpResponse {
    from_res(reset_profile_task(post).await)
}

async fn reset_profile_task(post: Json<V1TokenOnly>) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(&post.token).await?.v1_restrict_verified()?.v1_contains(&GMServices::Tex)?;

    reset_profile(account.id, GMServices::Tex).await?;

    Ok(V1Response::ProfileUpdated)
}
