use std::error::Error;

use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response, V1TokenOnly};

use crate::{functions::*, structs::*, traits::CollectionItem, *};

#[post("/invite")]
pub async fn invite(post: Json<V1TokenOnly>) -> HttpResponse {
    from_res(invite_task(post).await)
}

async fn invite_task(post: Json<V1TokenOnly>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();

    let account = match Account::find_by_token(&post.token).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    }
    .v1_restrict_verified()?;

    if !ALLOW_INVITE.get().unwrap() || !INVITE_WHITELIST.get().unwrap().contains(&account.id) {
        return Err(V1Error::FeatureDisabled.into());
    }

    let trigger_item = InviteTrigger {};

    let trigger = Trigger::new_from_action(Box::new(trigger_item), INVITE_DURATION.get().unwrap());

    trigger.init(DATABASE.get().unwrap()).await?;
    trigger.save_create(TRIGGERS.get().unwrap()).await?;

    Ok(V1Response::Invited { code: trigger.id })
}
