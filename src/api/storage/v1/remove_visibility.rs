use actix_web::{web::Json, *};

use std::{error::Error, path::PathBuf};

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::services::v1::{V1Error, V1PathOnly, V1Response};

#[post("/remove-visibility")]
pub async fn remove_visibility(post: Json<V1PathOnly>) -> HttpResponse {
    from_res(remove_visibility_task(post).await)
}

async fn remove_visibility_task(post: Json<V1PathOnly>) -> Result<V1Response, Box<dyn Error>> {
    let accounts = get_accounts(DATABASE.get().unwrap());
    let account = match Account::find_by_token(&post.token, &accounts).await? {
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

    let path_buf = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(account.id.to_string())
        .join(post.path.trim_start_matches('/'));

    if !editable(&path_buf) || has_dotdot(&path_buf) {
        return Err(V1Error::PermissionDenied.into());
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
