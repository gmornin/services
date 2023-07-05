use actix_web::{web::Json, *};
use std::error::Error;
use tokio::fs;

use crate::{functions::*, structs::*};

use goodmorning_bindings::services::v1::{V1Error, V1PathOnly, V1Response};

#[post("/delete")]
pub async fn delete(post: Json<V1PathOnly>) -> HttpResponse {
    from_res(delete_task(post).await)
}

async fn delete_task(post: Json<V1PathOnly>) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(&post.token).await?.v1_restrict_verified()?;

    let path_buf = get_user_dir(account.id, None).join(post.path.trim_start_matches('/'));

    if !editable(&path_buf) || has_dotdot(&path_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if !fs::try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if fs::metadata(&path_buf).await?.is_file() {
        fs::remove_file(&path_buf).await?;
    } else {
        fs::remove_dir_all(&path_buf).await?;
    }

    let mut visibilities = Visibilities::read_dir(path_buf.parent().unwrap()).await?;
    let file_name = path_buf
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
    if visibilities.0.contains_key(&file_name) {
        visibilities.0.remove(&file_name);
    }
    visibilities.save(path_buf.parent().unwrap()).await?;

    Ok(V1Response::Deleted)
}
