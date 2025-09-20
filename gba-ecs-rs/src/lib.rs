#![no_std]
extern crate alloc;

mod container;
mod entity;

pub use container::{
    ComponentContainer, DenseComponentContainer, DenseMarkerContainer, GetComponentContainer,
    MarkerContainer, SparseComponentContainer, SparseMarkerContainer,
};
pub use entity::Entity;
