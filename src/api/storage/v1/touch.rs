use actix_web::{web::Json, *};

use std::error::Error;
use tokio::fs;

use crate::{functions::*, structs::*};

use goodmorning_bindings::services::v1::{V1Error, V1PathOnly, V1Response};

#[post("/touch")]
pub async fn touch(post: Json<V1PathOnly>) -> HttpResponse {
    from_res(touch_task(post).await)
}

async fn touch_task(post: Json<V1PathOnly>) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    let path_buf = get_user_dir(account.id, None).join(post.path.trim_start_matches('/'));

    if !editable(&path_buf) || has_dotdot(&path_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if fs::try_exists(&path_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    fs::File::create(path_buf).await?;

    Ok(V1Response::FileItemCreated)
}
