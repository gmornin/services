use actix_web::{web::Json, *};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::{
    services::v1::{V1Error, V1PathOnly, V1Response},
    traits::ResTrait,
};

#[post("/delete")]
pub async fn delete(post: Json<V1PathOnly>) -> Json<V1Response> {
    Json(V1Response::from_res(
        delete_task(&post.path, &post.token).await,
    ))
}

async fn delete_task(path: &str, token: &str) -> Result<V1Response, Box<dyn Error>> {
    let accounts = get_accounts(DATABASE.get().unwrap());
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(V1Response::Error {
                kind: V1Error::InvalidToken,
            })
        }
    };

    if !account.verified {
        return Ok(V1Response::Error {
            kind: V1Error::NotVerified,
        });
    }

    let path_buf = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(account.id)
        .join(path.trim_start_matches('/'));

    if !editable(&path_buf) | has_dotdot(&path_buf) {
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
