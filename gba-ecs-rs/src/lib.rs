#![no_std]
#![feature(allocator_api)]

extern crate alloc;

mod container;
mod entity;
mod hash_container;
mod query;
mod vec_container;
mod world;
mod zip;

pub use container::{ComponentContainer, GetComponentContainer};
pub use entity::Entity;
pub use hash_container::HashComponentContainer;
pub use query::{Query, QueryMut};
pub use vec_container::VecComponentContainer;
pub use world::World;
pub use world::WorldContainer;
pub use zip::{zip, zip3, ZippedQuery2, ZippedQuery3};
