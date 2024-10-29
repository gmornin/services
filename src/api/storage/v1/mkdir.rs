use actix_web::{web::Json, *};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use goodmorning_bindings::{
    services::v1::{V1Error, V1PathOnly, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*};

#[post("/mkdir")]
pub async fn mkdir(post: Json<V1PathOnly>) -> Json<V1Response> {
    Json(V1Response::from_res(
        mkdir_task(&post.path, &post.token).await,
    ))
}

async fn mkdir_task(path: &str, token: &str) -> Result<V1Response, Box<dyn Error>> {
    let mut account = Account::v1_get_by_token(token)
        .await?
        .v1_restrict_verified()?;

    let mut user_path = PathBuf::from(path.trim_start_matches('/'));

    if let [_, "Shared", user, ..] = user_path
        .iter()
        .map(|s| s.to_str().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
    {
        account = if let Some(account) = Account::find_by_username(user.to_string()).await? {
            account.v1_restrict_verified()?
        } else {
            return Err(V1Error::FileNotFound.into());
        };
        user_path = [user_path.iter().next().unwrap()]
            .into_iter()
            .chain(user_path.iter().skip(3))
            .collect();
    }

    if !editable(&user_path, &account.services) || has_dotdot(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }

    let path_buf = get_user_dir(account.id, None).join(user_path);

    if fs::try_exists(&path_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    if !fs::try_exists(&path_buf.parent().unwrap()).await? {
        return Err(V1Error::FileNotFound.into());
    }

    // fs::create_dir_all(&path_buf).await?;
    fs::create_dir(&path_buf).await?;

    Ok(V1Response::FileItemCreated)
}
