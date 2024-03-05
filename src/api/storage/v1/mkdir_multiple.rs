use actix_web::{web::Json, *};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use goodmorning_bindings::{
    services::v1::{V1Error, V1MulpiplePaths, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*};

#[post("/mkdir-multiple")]
pub async fn mkdir_multiple(post: Json<V1MulpiplePaths>) -> Json<V1Response> {
    Json(V1Response::from_res(
        mkdir_multiple_task(&post.paths, &post.token).await,
    ))
}

async fn mkdir_multiple_task(
    paths: &Vec<String>,
    token: &str,
) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(token)
        .await?
        .v1_restrict_verified()?;

    let mut res = Vec::with_capacity(paths.len());

    for path in paths {
        res.push(V1Response::from_res(mkdir_single(path, &account).await))
    }

    Ok(V1Response::Multi { res })
}

async fn mkdir_single(path: &str, account: &Account) -> Result<V1Response, Box<dyn Error>> {
    let user_path = PathBuf::from(path.trim_start_matches('/'));

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
