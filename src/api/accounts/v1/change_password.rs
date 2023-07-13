use std::error::Error;

use crate::{functions::*, structs::*};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1ChangePassword, V1Error, V1Response};

#[post("/change-password")]
async fn rename(post: Json<V1ChangePassword>) -> HttpResponse {
    from_res(rename_task(post).await)
}

async fn rename_task(post: Json<V1ChangePassword>) -> Result<V1Response, Box<dyn Error>> {
    // let post = post.into_inner();
    // let accounts = ACCOUNTS.get().unwrap();
    //
    // let re = regex::Regex::new(r"^[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)*$").unwrap();
    // if !re.is_match(&post.new) || post.new.len() > 32 || post.new.len() < 3 {
    //     return Err(V1Error::InvalidUsername.into());
    // }
    //
    // let mut account = match Account::find_by_token(&post.token, accounts).await? {
    //     Some(account) => account,
    //     None => return Err(V1Error::InvalidToken.into()),
    // };
    //
    // if Account::find_by_username(post.new.clone(), accounts)
    //     .await?
    //     .is_some()
    // {
    //     return Err(V1Error::UsernameTaken.into());
    // }
    //
    // account.username = post.new;
    // account.save_replace(accounts).await?;
    //
    // Ok(V1Response::Renamed)

    let account = Account::v1_get_by_token(&post.token).await?;

    if !account.password_matches(&post.old) {
        return Err(V1Error::PasswordIncorrect.into());
    }

    // account.pa

    todo!()
}
