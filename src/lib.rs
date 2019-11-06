#![allow(clippy::new_without_default)]

#[macro_use]
extern crate downcast_rs;

mod storage;
pub use storage::*;
mod tuples;
mod world;
pub use world::*;
mod component;
pub use component::*;
mod archetype;
pub use archetype::*;
mod query;
pub use query::*;
mod update;
pub use update::*;
mod process;
pub use process::*;

#[cfg(test)]
mod tests;
