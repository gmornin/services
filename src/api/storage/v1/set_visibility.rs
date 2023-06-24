use actix_web::{web::Json, *};
use std::error::Error;
use tokio::fs;

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::services::v1::{V1Error, V1PathVisibility, V1Response};

#[post("/set-visibility")]
pub async fn set_visibility(post: Json<V1PathVisibility>) -> HttpResponse {
    from_res(set_visibility_task(post).await)
}

async fn set_visibility_task(post: Json<V1PathVisibility>) -> Result<V1Response, Box<dyn Error>> {
    let accounts = ACCOUNTS.get().unwrap();
    let account = match Account::find_by_token(&post.token, accounts).await? {
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

    let path_buf = get_user_dir(account.id, None).join(post.path.trim_start_matches('/'));

    if !editable(&path_buf) || has_dotdot(&path_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if !fs::try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    let file_name = path_buf.file_name().unwrap().to_str().unwrap();

    let mut visibilities = Visibilities::read_dir(path_buf.parent().unwrap()).await?;
    match visibilities.0.get(file_name) {
        Some(visibility) if visibility == &post.visibility.into() => {
            return Ok(V1Response::NothingChanged)
        }
        _ => {
            let _ = visibilities
                .0
                .insert(file_name.to_string(), post.visibility.into());
        }
    }
    visibilities.save(path_buf.parent().unwrap()).await?;

    Ok(V1Response::VisibilityChanged)
}
