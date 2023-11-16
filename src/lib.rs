#![allow(clippy::module_inception)]
#![feature(let_chains)]
#![feature(trait_upcasting)]
pub mod api;
pub mod functions;
pub mod pages;
#[allow(hidden_glob_reexports)]
mod r#static;
pub mod structs;
pub mod traits;
pub use r#static::*;

mod values;
pub use values::*;

mod tests;
pub use goodmorning_bindings as bindings;
