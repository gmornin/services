use std::{any::Any, error::Error};

use actix_web::{
    post,
    web::{Json, Query},
    HttpRequest, HttpResponse,
};
use goodmorning_bindings::{
    services::v1::{V1All3, V1Error, V1Response},
    structs::ServicesTriggerTypes,
};
use serde::Deserialize;

use crate::{functions::*, structs::*, traits::CollectionItem, *};

#[derive(Deserialize)]
pub struct Invite {
    pub invite: Option<String>,
}

#[post("/create")]
pub async fn create(post: Json<V1All3>, query: Query<Invite>, req: HttpRequest) -> HttpResponse {
    from_res(create_task(post, query.into_inner().invite, req).await)
}

async fn create_task(
    post: Json<V1All3>,
    invite: Option<String>,
    req: HttpRequest,
) -> Result<V1Response, Box<dyn Error>> {
    let mut allow_create = *ALLOW_REGISTER.get().unwrap()
        || CREATE_WHITELIST
            .get()
            .unwrap()
            .contains(&if *FORWARDED.get().unwrap() {
                req.connection_info()
                    .realip_remote_addr()
                    .unwrap()
                    .to_string()
            } else {
                req.connection_info().peer_addr().unwrap().to_string()
            });

    let mut invite_trigger = None;

    if let Some(invite) = invite {
        if let Some(trigger) = Trigger::find_by_id(invite, TRIGGERS.get().unwrap()).await? {
            let peek: Box<dyn Any> = match trigger.peek() {
                Some(trigger) => trigger,
                None => return Err(V1Error::TriggerNotFound.into()),
            };

            if (trigger.is_invalid()
                || !peek
                    .downcast_ref::<ServiceTriggerWrapper>()
                    .is_some_and(|wrapper| matches!(wrapper.0.value, ServicesTriggerTypes::Invite)))
                && !allow_create
            {
                return Err(V1Error::TriggerNotFound.into());
            }

            invite_trigger = Some(trigger);
            allow_create = true;
        }
    }

    if !allow_create {
        return Err(V1Error::FeatureDisabled.into());
    }

    let post = post.into_inner();

    if !Account::is_email_valid(&post.email) {
        return Err(V1Error::Blacklisted.into());
    }

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

    if let Some(invite_trigger) = invite_trigger {
        invite_trigger.trigger(DATABASE.get().unwrap()).await?;
    }

    account.save_create(ACCOUNTS.get().unwrap()).await?;

    Ok(V1Response::Created {
        id: account.id,
        token: account.token,
        verify: *VERIFICATION.get().unwrap(),
    })
}
