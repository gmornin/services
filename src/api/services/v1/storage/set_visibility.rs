use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use std::{error::Error, path::PathBuf};

use crate::{functions::*, structs::*};

use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
};

#[derive(Deserialize)]
struct StaticPath {
    path: String,
    token: String,
}

#[derive(Deserialize)]
struct SetVisibility {
    pub new: ItemVisibility,
}

#[post("/set_visibility/{path:.*}")]
pub async fn set_visibility(
    path: Path<StaticPath>,
    db: Data<Database>,
    post: Json<SetVisibility>,
) -> Json<V1Response> {
    Json(V1Response::from_res(
        set_visibility_task(&path.path, &path.token, &db, &post).await,
    ))
}

async fn set_visibility_task(
    path: &str,
    token: &str,
    db: &Database,
    post: &SetVisibility,
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
        Some(visibility) if visibility == &post.new => return Ok(V1Response::NothingChanged),
        _ => {
            let _ = visibilities.0.insert(file_name.to_string(), post.new);
        }
    }
    visibilities.save(path_buf.parent().unwrap()).await?;

    Ok(V1Response::VisibilityChanged)
}
