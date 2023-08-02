use std::error::Error;

use crate::{functions::*, structs::*};
use actix_web::{
    post,
    web::{self, Json},
    HttpResponse,
};
use goodmorning_bindings::services::v1::{V1Response, V1Unqueue};

#[post("/unqueue")]
async fn unqueue(post: Json<V1Unqueue>, userjobs: web::Data<Jobs>) -> HttpResponse {
    from_res(unqueue_task(post, userjobs).await)
}

async fn unqueue_task(
    post: Json<V1Unqueue>,
    userjobs: web::Data<Jobs>,
) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    userjobs.unqueue(account.id, post.id)?;
    Ok(V1Response::Unqueued)
}
