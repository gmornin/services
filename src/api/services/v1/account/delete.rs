use std::error::Error;

use actix_web::{
    post,
    web::{Data, Json},
};
use goodmorning_bindings::{
    services::v1::{V1Error, V1Response, V1TokenOnly},
    traits::ResTrait,
};
use mongodb::Database;

use crate::{functions::*, structs::*, traits::CollectionItem};

#[post("/delete")]
async fn delete(post: Json<V1TokenOnly>, db: Data<Database>) -> Json<V1Response> {
    Json(V1Response::from_res(delete_task(post, db).await))
}

async fn delete_task(
    post: Json<V1TokenOnly>,
    db: Data<Database>,
) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(&db);

    let account = match Account::find_by_token(&post.token, &accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    account.delete(&accounts).await?;

    Ok(V1Response::Deleted)
}
