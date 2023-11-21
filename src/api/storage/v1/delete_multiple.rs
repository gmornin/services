use actix_web::{web::Json, *};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use crate::{functions::*, structs::*};

use goodmorning_bindings::{
    services::v1::{V1Error, V1MulpiplePaths, V1Response},
    traits::ResTrait,
};

#[post("/delete-multiple")]
pub async fn delete_multiple(post: Json<V1MulpiplePaths>) -> HttpResponse {
    from_res(delete_multiple_task(post).await)
}

async fn delete_multiple_task(post: Json<V1MulpiplePaths>) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    let mut res = Vec::with_capacity(post.paths.len());

    for path in post.paths.iter() {
        res.push(V1Response::from_res(delete_single(path, &account).await))
    }

    Ok(V1Response::Multi { res })
}

async fn delete_single(path: &str, account: &Account) -> Result<V1Response, Box<dyn Error>> {
    let user_path = PathBuf::from(path.trim_start_matches('/'));
    if !editable(&user_path, &account.services) || has_dotdot(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }
    let path_buf = get_user_dir(account.id, None).join(user_path);

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

    Ok(V1Response::FileItemDeleted)
}
