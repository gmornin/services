use std::error::Error;

use crate::functions::*;
use actix_web::{get, web, HttpResponse};
use goodmorning_bindings::services::v1::V1Response;

#[get("/profile/{id}")]
async fn profile(id: web::Path<i64>) -> HttpResponse {
    from_res(profile_task(id).await)
}

async fn profile_task(id: web::Path<i64>) -> Result<V1Response, Box<dyn Error>> {
    Ok(V1Response::Profile {
        profile: read_profile(*id, "gmt").await?,
    })
}
