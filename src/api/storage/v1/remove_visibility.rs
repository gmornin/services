use actix_web::{web::Json, *};

use std::{error::Error, path::PathBuf};

use crate::{functions::*, structs::*};

use goodmorning_bindings::services::v1::{V1Error, V1PathOnly, V1Response};

#[post("/remove-visibility")]
pub async fn remove_visibility(post: Json<V1PathOnly>) -> HttpResponse {
    from_res(remove_visibility_task(post).await)
}

async fn remove_visibility_task(post: Json<V1PathOnly>) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    let user_path = PathBuf::from(post.path.trim_start_matches('/'));

    if !editable(&user_path, &account.services) || has_dotdot(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }

    let path_buf = get_user_dir(account.id, None).join(user_path);

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
