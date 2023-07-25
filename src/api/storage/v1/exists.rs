use actix_web::{web::Json, *};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use goodmorning_bindings::{
    services::v1::{V1Error, V1PathOnly, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*};

#[post("/exists")]
pub async fn exists(post: Json<V1PathOnly>) -> Json<V1Response> {
    Json(V1Response::from_res(
        exists_task(&post.path, &post.token).await,
    ))
}

async fn exists_task(path: &str, token: &str) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(token)
        .await?
        .v1_restrict_verified()?;

    let user_path = PathBuf::from(path.trim_start_matches('/'));

    if is_bson(&user_path) || has_dotdot(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }

    let path_buf = get_user_dir(account.id, None).join(user_path);

    Ok(V1Response::Exists {
        value: fs::try_exists(path_buf).await?,
    })
}
