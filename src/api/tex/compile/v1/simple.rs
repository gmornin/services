use std::error::Error;

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
    let accounts = ACCOUNTS.get().unwrap();

    let account = match Account::find_by_token(&post.token, accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if !account.services.contains(&GMServices::Tex) {
        return Err(V1Error::NotCreated.into());
    }

    let source =
        get_user_dir(account.id, Some(GMServices::Tex)).join(post.path.trim_start_matches('/'));

    let res = match (post.from, post.to) {
        (FromFormat::Markdown, ToFormat::Html) => jobs
            .run_with_limit(
                account.id,
                SingleTask::Compile {
                    from: FromFormat::Markdown,
                    to: ToFormat::Html,
                    source,
                },
                *MAX_CONCURRENT.get().unwrap(),
            )
            .await
            .unwrap(),
    };

    Ok(res)
}
