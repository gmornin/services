use actix_web::{
    web::{Data, Json},
    *,
};
use mongodb::Database;
use std::{error::Error, path::PathBuf};
use tokio::fs;

use goodmorning_bindings::{
    services::v1::{V1Error, V1PathOnly, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*, *};

#[post("/mkdir")]
pub async fn mkdir(post: Json<V1PathOnly>, db: Data<Database>) -> Json<V1Response> {
    Json(V1Response::from_res(
        mkdir_task(&post.path, &post.token, &db).await,
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

    if !account.verified {
        return Ok(V1Response::Error {
            kind: V1Error::NotVerified,
        });
    }

    let path_buf = PathBuf::from(USERCONTENT.as_str()).join(&account.id).join(path.trim_start_matches('/'));

    if !editable(&path_buf) | has_dotdot(&path_buf) {
        return Err(V1Error::PermissionDenied.into());
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
