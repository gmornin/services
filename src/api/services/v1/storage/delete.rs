use actix_web::{
    web::{Data, Json},
    *,
};
use mongodb::Database;
use std::{error::Error, path::PathBuf};
use tokio::fs;

use crate::{functions::*, structs::*};

use goodmorning_bindings::{
    services::v1::{V1Error, V1PathOnly, V1Response},
    traits::ResTrait,
};

#[post("/delete")]
pub async fn delete(db: Data<Database>, post: Json<V1PathOnly>) -> Json<V1Response> {
    Json(V1Response::from_res(
        delete_task(&post.path, &post.token, &db).await,
    ))
}

async fn delete_task(path: &str, token: &str, db: &Database) -> Result<V1Response, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(V1Response::Error {
                kind: V1Error::InvalidToken,
            })
        }
    };

    let path_buf = PathBuf::from(format!("usercontent/{}/{}", account.id, path));

    if !editable(&path_buf) {
        return Err(V1Error::NotEditable.into());
    }

    if fs::try_exists(&path_buf).await? {
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
