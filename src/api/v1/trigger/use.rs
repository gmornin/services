use std::error::Error;

use actix_web::{
    get,
    web::{Data, Json, Path},
};
use mongodb::Database;

use super::{ErrorKind, Responses};
use crate::{functions::*, structs::*, traits::CollectionItem, *};

#[get("/use/{id}")]
async fn r#use(id: Path<String>, db: Data<Database>) -> Json<Responses> {
    match use_task(&id.into_inner(), db).await {
        Ok(res) => Json(res),
        Err(e) => Json(Responses::Error {
            kind: match e.downcast::<ErrorKind>() {
                Ok(downcasted) => *downcasted,
                Err(e) => ErrorKind::External(e.to_string()),
            },
        }),
    }
}

async fn use_task(id: &str, db: Data<Database>) -> Result<Responses, Box<dyn Error>> {
    let triggers = get_triggers(&db);
    let trigger = match Trigger::find_by_id(id, &triggers).await? {
        Some(trigger) => trigger,
        None => return Err(ErrorKind::NotFound.into()),
    };

    if trigger.is_invalid() {
        trigger.delete(&triggers).await?;
        return Err(ErrorKind::NotFound.into());
    }

    trigger.trigger(&db).await?;

    Ok(Responses::Triggered)
}
