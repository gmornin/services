use std::{error::Error, path::PathBuf};

use crate::{functions::*, structs::*, *};
use actix_web::{
    post,
    web::{self, Json},
    HttpResponse,
};
use goodmorning_bindings::services::v1::*;

#[post("/simple")]
async fn simple(post: Json<V1Compile>, jobs: web::Data<Jobs>) -> HttpResponse {
    from_res(simple_task(post, jobs).await)
}

async fn simple_task(
    post: Json<V1Compile>,
    jobs: web::Data<Jobs>,
) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?
        .v1_contains(&GMServices::Tex)?;

    let user_path = PathBuf::from(post.path.trim_start_matches('/'));
    let source = get_user_dir(account.id, Some(GMServices::Tex)).join(&user_path);

    if has_dotdot(&source) && !editable(&source) {
        return Err(V1Error::PermissionDenied.into());
    }

    let res = jobs
        .run_with_limit(
            account.id,
            SingleTask::Compile {
                from: post.from,
                to: post.to,
                compiler: post.compiler.unwrap_or_default(),
                source,
                user_path,
            },
            *MAX_CONCURRENT.get().unwrap(),
        )
        .await?;

    Ok(res)
}
