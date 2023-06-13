use std::{error::Error, path::PathBuf};

use crate::{functions::*, structs::*, traits::CollectionItem, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response, V1TokenOnly};
use tokio::fs;

#[post("/create")]
async fn rename(post: Json<V1TokenOnly>) -> HttpResponse {
    from_res(rename_task(post).await)
}

async fn rename_task(post: Json<V1TokenOnly>) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(DATABASE.get().unwrap());

    let mut account = match Account::find_by_token(&post.token, &accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if account.services.contains(&GMServices::Tex) {
        return Err(V1Error::AlreadyCreated.into());
    }

    let mut path = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(account.id.to_string())
        .join("tex");
    fs::create_dir_all(&path).await?;

    path.push(".system");
    fs::create_dir_all(&path).await?;

    account.services.push(GMServices::Tex);
    account.save_replace(&accounts).await?;

    Ok(V1Response::ServiceCreated)
}
