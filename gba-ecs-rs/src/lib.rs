#![no_std]
extern crate alloc;

mod container;
mod entity;
mod zip;

pub use container::{ComponentContainer, VecComponentContainer, GetComponentContainer};
pub use entity::Entity;
pub use zip::{zip, zip3, ZippedQuery2, ZippedQuery3};
