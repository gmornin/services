use std::error::Error;

use crate::{functions::*, structs::*, *};
use actix_web::{post, web::Json, HttpResponse};
use goodmorning_bindings::{
    services::v1::{V1Error, V1ProfileOnly, V1Response},
    structs::ProfileDetail,
};

#[post("/set-profile")]
async fn set_profile(post: Json<V1ProfileOnly>) -> HttpResponse {
    from_res(set_profile_task(post).await)
}

async fn set_profile_task(post: Json<V1ProfileOnly>) -> Result<V1Response, Box<dyn Error>> {
    let accounts = ACCOUNTS.get().unwrap();

    let account = match Account::find_by_token(&post.token, accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    let profile = &post.profile;

    if profile.description.len() > 2000 {
        return Err(V1Error::ExceedsMaximumLength.into());
    }

    if profile.details.len() > 20 {
        return Err(V1Error::TooManyProfileDetails.into());
    }

    if profile
        .details
        .iter()
        .filter(|detail| {
            matches!(
                detail,
                ProfileDetail::CakeDay { .. } | ProfileDetail::BirthDay { .. }
            )
        })
        .count()
        > 1
    {
        return Err(V1Error::BirthCakeConflict.into());
    }

    for (i, detail) in profile.details.iter().enumerate() {
        if !detail.validate() {
            return Err(V1Error::InvalidDetail { index: i as u8 }.into());
        }
    }

    save_profile(&post.profile, account.id, "tex").await?;

    Ok(V1Response::ProfileUpdated)
}