use async_trait::async_trait;
use dyn_clone::DynClone;
use goodmorning_bindings::services::v1::V1Response;
use goodmorning_bindings::traits::TaskRes;
use std::{error::Error, fmt::Debug};

#[async_trait]
pub trait TaskItem: DynClone + Debug + Sync + Send {
    #[cfg(feature = "v1")]
    async fn run_v1(&self, taskid: u64) -> Result<V1Response, Box<dyn Error>>;
    #[cfg(feature = "v1")]
    fn to_v1(&self) -> Box<dyn TaskRes>;
}

dyn_clone::clone_trait_object!(TaskItem);
