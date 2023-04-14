use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::{error::Error, path::PathBuf};

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

#[get("/remove_visibility/{path:.*}")]
pub async fn remove_visibility(
    path: Path<StaticPath>,
    db: Data<Database>,
) -> Json<GMResponses> {
    Json(to_res(
        remove_visibility_task(&path.path, &path.token, &db).await,
    ))
}

async fn remove_visibility_task(
    path: &str,
    token: &str,
    db: &Database,
) -> Result<GMResponses, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(GMResponses::Error {
                kind: GMError::InvalidToken,
            })
        }
    };

    let path = PathBuf::from(path);
    if path.parent().is_none() {
        return Err(GMError::NoParent.into());
    }

    let path_buf = PathBuf::from(format!("usercontent/{}", account.id)).join(path);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
  
    let mut visibilities = Visibilities::read_dir(path_buf.parent().unwrap()).await?;
    match visibilities.0.get(file_name) {
        None => return Ok(GMResponses::NothingChanged),
        _ => {
            let _ = visibilities.0.remove(file_name);
        }
    }
    visibilities.save(path_buf.parent().unwrap()).await?;

    Ok(GMResponses::VisibilityChanged)
}
