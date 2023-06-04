use actix_web::{
    web::{Data, Json},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::{error::Error, path::PathBuf};
use tokio::fs;

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
};

#[derive(Deserialize)]
struct Touch {
    path: String,
    token: String,
}

#[get("/touch")]
pub async fn touch(post: Json<Touch>, db: Data<Database>) -> Json<V1Response> {
    Json(V1Response::from_res(
        touch_task(&post.path, &post.token, &db).await,
    ))
}

async fn touch_task(path: &str, token: &str, db: &Database) -> Result<V1Response, Box<dyn Error>> {
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

    if !fs::try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if fs::try_exists(&path_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    fs::File::create(path_buf).await?;

    Ok(V1Response::FileItemCreated {
        path: format!("/{}/{}", account.id, path),
    })
}
