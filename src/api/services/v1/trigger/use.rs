use std::error::Error;

use actix_web::{
    get,
    web::{Data, Json, Path},
};
use mongodb::Database;

use crate::{api::services::v1::*, functions::*, structs::*, traits::CollectionItem, *};

#[get("/use/{id}")]
async fn r#use(id: Path<String>, db: Data<Database>) -> Json<GMResponses> {
    Json(to_res(use_task(&id.into_inner(), db).await))
}

async fn use_task(id: &str, db: Data<Database>) -> Result<GMResponses, Box<dyn Error>> {
    let triggers = get_triggers(&db);
    let trigger = match Trigger::find_by_id(id, &triggers).await? {
        Some(trigger) => trigger,
        None => return Err(GMError::TriggerNotFound.into()),
    };

    if trigger.is_invalid() {
        trigger.delete(&triggers).await?;
        return Err(GMError::TriggerNotFound.into());
    }

    trigger.trigger(&db).await?;

    Ok(GMResponses::Triggered)
}
