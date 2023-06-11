use std::error::Error;

use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1All3, V1Error, V1Response};

use crate::{functions::*, structs::*, traits::CollectionItem, *};

#[post("/create")]
async fn create(post: Json<V1All3>) -> HttpResponse {
    from_res(create_task(post).await)
}

async fn create_task(post: Json<V1All3>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(DATABASE.get().unwrap());
    let triggers = get_triggers(DATABASE.get().unwrap());

    let re = regex::Regex::new(r"^[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)*$").unwrap();
    if !re.is_match(&post.username) || post.username.len() > 32 || post.username.len() < 3 {
        return Err(V1Error::InvalidUsername.into());
    }

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

    let account = Account::new(
        post.username,
        &post.password,
        &post.email,
        DATABASE.get().unwrap(),
    )
    .await?;

    account.save_create(&accounts).await?;

    let trigger = Trigger::new_from_action(
        Box::new(account.email_verification()),
        EMAIL_VERIFICATION_DURATION.get().unwrap(),
    );
    trigger.init(DATABASE.get().unwrap()).await?;
    trigger.save_create(&triggers).await?;

    Ok(V1Response::Created {
        id: account.id,
        token: account.token,
    })
}
