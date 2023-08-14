use async_trait::async_trait;
use dyn_clone::DynClone;
use goodmorning_bindings::structs::ApiVer;
use goodmorning_bindings::structs::CommonRes;
use goodmorning_bindings::traits::SerdeAny;
use std::fmt::Debug;

#[async_trait]
pub trait TaskItem: DynClone + Debug + Sync + Send {
    // #[cfg(feature = "v1")]
    async fn run(&self, ver: &ApiVer, taskid: u64) -> CommonRes;
    // #[cfg(feature = "v1")]
    fn to(&self, ver: &ApiVer) -> Box<dyn SerdeAny>;
}

dyn_clone::clone_trait_object!(TaskItem);
