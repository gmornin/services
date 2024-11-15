use async_trait::async_trait;
use goodmorning_bindings::structs::{ServicesTriggerTypes, ServicesTriggers};
use serde::{Deserialize, Serialize};

use crate::{
    structs::ServiceTriggerWrapper,
    traits::{Peekable, Triggerable},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InviteTrigger;

#[typetag::serde]
#[async_trait]
impl Triggerable for InviteTrigger {
    fn peek(&self, _id: &str, expiry: u64) -> Option<Box<dyn Peekable>> {
        Some(Box::new(ServiceTriggerWrapper(ServicesTriggers {
            expiry,
            value: ServicesTriggerTypes::Invite,
        })))
    }
}
