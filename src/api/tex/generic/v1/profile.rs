use std::error::Error;

use crate::{
    functions::*,
    structs::{Account, GMServices},
    ACCOUNTS,
};
use actix_web::{get, web, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1Response};

#[get("/profile/id/{id}")]
async fn profile(id: web::Path<i64>) -> HttpResponse {
    from_res(profile_task(id).await)
}

async fn profile_task(id: web::Path<i64>) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_id(*id)
        .await?
        .v1_contains(&GMServices::Tex)?;
    let profile_customisable = read_profile(account.id, GMServices::Tex).await?;

    Ok(V1Response::Profile {
        profile: profile_customisable,
        account: to_profile_acccount(account),
    })
}

#[get("/profile/name/{name}")]
async fn profile_by_name(name: web::Path<String>) -> HttpResponse {
    from_res(profile_by_name_task(name).await)
}

async fn profile_by_name_task(name: web::Path<String>) -> Result<V1Response, Box<dyn Error>> {
    let account = match Account::find_by_username(name.to_string(), ACCOUNTS.get().unwrap()).await?
    {
        Some(account) => account,
        None => return Err(V1Error::NoSuchUser.into()),
    };
    let profile_customisable = read_profile(account.id, GMServices::Tex).await?;

    Ok(V1Response::Profile {
        profile: profile_customisable,
        account: to_profile_acccount(account),
    })
}

#[get("/profile-only/id/{id}")]
async fn profile_only(id: web::Path<i64>) -> HttpResponse {
    from_res(profile_only_task(id).await)
}

async fn profile_only_task(id: web::Path<i64>) -> Result<V1Response, Box<dyn Error>> {
    let profile_customisable = read_profile(*id, GMServices::Tex).await?;

    Ok(V1Response::ProfileOnly {
        profile: profile_customisable,
    })
}
