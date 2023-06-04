use std::error::Error;

use actix_web::{post, web::Json};
use goodmorning_bindings::{
    services::v1::{V1All3, V1Error, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*, traits::CollectionItem, *};

#[post("/create")]
async fn create(post: Json<V1All3>) -> Json<V1Response> {
    Json(V1Response::from_res(create_task(post).await))
}

async fn create_task(post: Json<V1All3>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(DATABASE.get().unwrap());
    let triggers = get_triggers(DATABASE.get().unwrap());

    if Account::find_by_username(post.username.clone(), &accounts)
        .await?
        .is_some()
    {
        return Err(V1Error::UsernameTaken.into());
    }

    if Account::find_by_email(&post.email, &accounts)
        .await?
        .is_some()
    {
        return Err(V1Error::EmailTaken.into());
    }

    let account = Account::new(post.username, &post.password, &post.email);

    let trigger = Trigger::new_from_action(
        Box::new(account.email_verification()),
        EMAIL_VERIFICATION_DURATION.get().unwrap(),
    );
    trigger.init(DATABASE.get().unwrap()).await?;
    trigger.save_create(&triggers).await?;

    account.save_create(&accounts).await?;

    Ok(V1Response::Created {
        id: account.id,
        token: account.token,
    })
}
