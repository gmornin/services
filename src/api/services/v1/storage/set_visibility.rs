use actix_web::{
    web::{Data, Json},
    *,
};
use mongodb::Database;
use tokio::fs;
use std::{error::Error, path::PathBuf};

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::{
    services::v1::{V1Error, V1PathVisibility, V1Response, V1Visibility},
    traits::ResTrait,
};

#[post("/set_visibility")]
pub async fn set_visibility(db: Data<Database>, post: Json<V1PathVisibility>) -> Json<V1Response> {
    Json(V1Response::from_res(
        set_visibility_task(&post.path, &post.token, post.visibility, &db).await,
    ))
}

async fn set_visibility_task(
    path: &str,
    token: &str,
    new: V1Visibility,
    db: &Database,
) -> Result<V1Response, Box<dyn Error>> {
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

    let file_name = path_buf.file_name().unwrap().to_str().unwrap();

    let mut visibilities = Visibilities::read_dir(path_buf.parent().unwrap()).await?;
    match visibilities.0.get(file_name) {
        Some(visibility) if visibility == &new.visibility.into() => {
            return Ok(V1Response::NothingChanged)
        }
        _ => {
            let _ = visibilities
                .0
                .insert(file_name.to_string(), new.visibility.into());
        }
    }
    visibilities.save(path_buf.parent().unwrap()).await?;

    Ok(V1Response::VisibilityChanged)
}
