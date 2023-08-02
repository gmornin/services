use std::error::Error;

use crate::{functions::*, structs::*};
use actix_web::{
    post,
    web::{self, Json},
    HttpResponse,
};
use goodmorning_bindings::services::v1::{V1Response, V1TokenOnly};

#[post("/jobs")]
async fn jobs(post: Json<V1TokenOnly>, userjobs: web::Data<Jobs>) -> HttpResponse {
    from_res(jobs_task(post, userjobs).await)
}

async fn jobs_task(
    post: Json<V1TokenOnly>,
    userjobs: web::Data<Jobs>,
) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    let arc = userjobs.get(account.id);
    let userjobs = &*arc.lock().unwrap();

    Ok(V1Response::Jobs {
        current: userjobs.current.iter().map(SingleJob::to_v1).collect(),
        queue: userjobs.queue.iter().map(|job| job.job.to_v1()).collect(),
    })
}
