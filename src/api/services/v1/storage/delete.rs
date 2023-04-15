use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::{error::Error, path::PathBuf};
use tokio::fs;

use crate::{
    api::services::v1::*,
    functions::*,
    structs::*,
};

#[derive(Deserialize)]
struct StaticPath {
    path: String,
    token: String,
}

#[get("/delete/{path:.*}")]
pub async fn delete(path: Path<StaticPath>, db: Data<Database>) -> Json<GMResponses> {
    Json(to_res(delete_task(&path.path, &path.token, &db).await))
}

async fn delete_task(path: &str, token: &str, db: &Database) -> Result<GMResponses, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(GMResponses::Error {
                kind: GMError::InvalidToken,
            })
        }
    };

    let path_buf = PathBuf::from(format!("usercontent/{}/{}", account.id, path));

    if !editable(&path_buf) {
        return Err(GMError::NotEditable.into());
    }

    if fs::try_exists(&path_buf).await? {
        return Err(GMError::FileNotFound.into());
    }

    if fs::metadata(&path_buf).await?.is_file() {
        fs::remove_file(path_buf).await?;
    } else {
        fs::remove_dir_all(path_buf).await?;
    }

    Ok(GMResponses::Deleted)
}
