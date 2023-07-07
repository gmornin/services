use actix_web::{web::Json, *};
use std::error::Error;
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
    let account = Account::v1_get_by_token(token)
        .await?
        .v1_restrict_verified()?;

    let path_buf = get_user_dir(account.id, None).join(path.trim_start_matches('/'));

    if !editable(&path_buf) || has_dotdot(&path_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if fs::try_exists(&path_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    // fs::create_dir_all(&path_buf).await?;
    fs::create_dir(&path_buf).await?;

    Ok(V1Response::FileItemCreated)
}
