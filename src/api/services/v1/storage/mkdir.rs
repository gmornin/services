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

#[get("/mkdir/{path:.*}")]
pub async fn mkdir(path: Path<StaticPath>, db: Data<Database>) -> Json<GMResponses> {
    Json(to_res(mkdir_task(&path.path, &path.token, &db).await))
}

async fn mkdir_task(path: &str, token: &str, db: &Database) -> Result<GMResponses, Box<dyn Error>> {
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

    if fs::try_exists(&path_buf).await? {
        return Err(GMError::PathOccupied.into());
    }

    // fs::create_dir_all(&path_buf).await?;
    fs::create_dir(&path_buf).await?;

    Ok(GMResponses::Overwritten {
        path: format!("/{}/{}", account.id, path),
    })
}
