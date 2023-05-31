use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::{error::Error, path::PathBuf};
use tokio::fs;

use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*};

#[derive(Deserialize)]
struct StaticPath {
    path: String,
    token: String,
}

#[get("/mkdir/{path:.*}")]
pub async fn mkdir(path: Path<StaticPath>, db: Data<Database>) -> Json<V1Response> {
    Json(V1Response::from_res(
        mkdir_task(&path.path, &path.token, &db).await,
    ))
}

async fn mkdir_task(path: &str, token: &str, db: &Database) -> Result<V1Response, Box<dyn Error>> {
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
        return Err(V1Error::PathOccupied.into());
    }

    // fs::create_dir_all(&path_buf).await?;
    fs::create_dir(&path_buf).await?;

    Ok(V1Response::FileItemCreated {
        path: format!("/{}/{}", account.id, path),
    })
}
