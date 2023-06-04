#[allow(private_in_public)]
pub mod api;
pub mod functions;
pub mod structs;
pub mod traits;

mod values;
use lazy_static::lazy_static;
use std::env;
pub use values::*;

mod tests;

lazy_static! {
    pub static ref STORAGE: String = env::var("STORAGE_PATH").unwrap();
    pub static ref USERCONTENT: String = env::var("USERCONTENT_PATH").unwrap();
}
