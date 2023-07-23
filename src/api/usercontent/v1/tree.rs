use actix_web::{web::Path, *};
use std::error::Error;

use crate::{functions::*, structs::*};

use goodmorning_bindings::services::v1::V1Response;

#[get("/tree/{id}/{path:.*}")]
pub async fn tree(path: Path<(i64, String)>) -> HttpResponse {
    from_res(tree_task(path).await)
}

async fn tree_task(path: Path<(i64, String)>) -> Result<V1Response, Box<dyn Error>> {
    let (id, path) = path.into_inner();

    let src = get_user_dir(id, None).join(path);
    Ok(V1Response::Tree {
        content: dirtree(&src, false, Visibilities::visibility(&src).await?).await?,
    })
}
