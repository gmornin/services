use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use tokio::fs;
use std::{error::Error, path::PathBuf, ffi::OsStr};


use crate::{
    api::services::v1::*,
    functions::*,
    structs::*,
};

use super::r#move::*;

#[derive(Deserialize)]
struct StaticPath {
    path: String,
    token: String,
}

#[post("/move_overwrite/{path:.*}")]
pub async fn move_overwrite(
    path: Path<StaticPath>,
    db: Data<Database>,
    post: Json<MoveFrom>,
) -> Json<GMResponses> {
    Json(to_res(
        move_overwrite_task(&path.path, &path.token, &db, &post).await,
    ))
}

async fn move_overwrite_task(path: &str, token: &str, db: &Database, post: &MoveFrom) -> Result<GMResponses, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(GMResponses::Error {
                kind: GMError::InvalidToken,
            })
        }
    };

    let path_buf = PathBuf::from(format!("usercontent/{}", account.id)).join(path);

    if !editable(&path_buf) {
        return Err(GMError::NotEditable.into());
    }

    let from_buf = PathBuf::from(format!("usercontent/{}/{}",account.id, post.from));

    if from_buf.iter().any(|section| section == OsStr::new(".") || section == OsStr::new("..")) {
        return Err(GMError::FileNotFound.into());
    }

    if !editable(&from_buf) {
        return Err(GMError::NotEditable.into());
    }

    if !(fs::try_exists(&from_buf).await? && fs::try_exists(&path_buf).await?) {
        return Err(GMError::FileNotFound.into());
    }

    if fs::metadata(&path_buf).await?.is_dir() {
        fs::remove_dir_all(&path_buf).await?;
    } else {
        fs::remove_file(&path_buf).await?;
    }

    fs::rename(from_buf, path_buf).await?;

    Ok(GMResponses::Moved { path: format!("/{}/{}", account.id, path) })
}
