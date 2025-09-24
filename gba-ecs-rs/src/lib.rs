#![no_std]
extern crate alloc;

mod container;
mod entity;
mod zip;

pub use container::{
    ComponentContainer, DenseComponentContainer, DenseMarkerContainer, GetComponentContainer,
    MarkerContainer, SparseComponentContainer, SparseMarkerContainer,
};
pub use entity::Entity;
pub use zip::{
    zip, zip3, zip3_mut, zip3_mut_mut, zip3_mut_mut_mut, zip_mut, zip_mut_mut, ZippedQuery2,
    ZippedQuery3,
};
