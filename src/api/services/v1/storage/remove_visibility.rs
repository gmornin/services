use actix_web::{
    web::{Data, Json},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::{error::Error, path::PathBuf};

use crate::{functions::*, structs::*};

#[derive(Deserialize)]
struct RemoveVis {
    path: String,
    token: String,
}

use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
};

#[post("/remove_visibility")]
pub async fn remove_visibility(post: Json<RemoveVis>, db: Data<Database>) -> Json<V1Response> {
    Json(V1Response::from_res(
        remove_visibility_task(&post.path, &post.token, &db).await,
    ))
}

async fn remove_visibility_task(
    path: &str,
    token: &str,
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

    let path_buf = PathBuf::from(format!("usercontent/{}", account.id)).join(path);

    if !editable(&path_buf) {
        return Err(V1Error::NotEditable.into());
    }

    let file_name = path_buf.file_name().unwrap().to_str().unwrap();

    let mut visibilities = Visibilities::read_dir(path_buf.parent().unwrap()).await?;
    match visibilities.0.get(file_name) {
        None => return Ok(V1Response::NothingChanged),
        _ => {
            let _ = visibilities.0.remove(file_name);
        }
    }
    visibilities.save(path_buf.parent().unwrap()).await?;

    Ok(V1Response::VisibilityChanged)
}
