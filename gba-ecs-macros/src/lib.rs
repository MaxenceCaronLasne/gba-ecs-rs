//! # GBA ECS Macros
//!
//! This crate provides procedural macros for the GBA ECS (Entity Component System) library.
//! It's designed to work in `no_std` environments, specifically for Game Boy Advance development.
//!
//! ## Macros
//!
//! - [`Component`]: A derive macro that implements the `Component` trait for structs
//! - [`define_world`]: A function-like macro that generates a complete ECS World with storage for specified components
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use gba_ecs_macros::{Component, define_world};
//!
//! // Define components by deriving the Component trait
//! #[derive(Component)]
//! struct Position { x: f32, y: f32 }
//!
//! #[derive(Component)]
//! struct Velocity { dx: f32, dy: f32 }
//!
//! // Generate a World struct with storage for these components
//! define_world!(GameWorld {
//!     Position,
//!     Velocity,
//! });
//!
//! // Use the generated world
//! let mut world = GameWorld::new();
//! let entity = world.spawn_entity();
//! world.add_component(entity, Position { x: 10.0, y: 20.0 });
//! ```

use proc_macro::TokenStream;

mod component;
mod world;

/// Derive macro for implementing the `Component` trait.
///
/// This macro can only be applied to structs and will generate an empty implementation
/// of the `Component` trait. The `Component` trait serves as a marker trait to identify
/// types that can be used as ECS components.
///
/// # Examples
///
/// ```rust,ignore
/// use gba_ecs_macros::Component;
///
/// #[derive(Component)]
/// struct Position {
///     x: f32,
///     y: f32,
/// }
///
/// #[derive(Component)]
/// struct Health {
///     current: i32,
///     max: i32,
/// }
/// ```
///
/// # Errors
///
/// This macro will produce a compile error if applied to anything other than a struct:
///
/// ```rust,ignore
/// #[derive(Component)]
/// enum InvalidComponent { } // This will cause a compile error
/// ```
#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    component::derive_component_impl(input)
}

/// Generates a complete ECS World struct with storage for the specified components.
///
/// This macro creates a world struct that can manage entities and their components.
/// For each component type listed, it generates:
/// - A storage field to hold component data
/// - Implementation of `GetStorage` trait for type-safe component access
/// - Methods for entity and component management
///
/// # Syntax
///
/// ```rust,ignore
/// define_world!(WorldName {
///     Component1,
///     Component2,
///     // ... more components
/// });
/// ```
///
/// # Generated Methods
///
/// The generated world struct includes these public methods:
/// - `new()` - Creates a new empty world
/// - `spawn_entity()` - Creates a new entity and returns its ID
/// - `add_component(entity, component)` - Adds a component to an entity
/// - `remove_component::<T>(entity)` - Removes and returns a component from an entity
/// - `get_component::<T>(entity)` - Gets a read-only reference to a component
/// - `get_component_mut::<T>(entity)` - Gets a mutable reference to a component
///
/// # Examples
///
/// ```rust,ignore
/// use gba_ecs_macros::{Component, define_world};
///
/// #[derive(Component)]
/// struct Position { x: f32, y: f32 }
///
/// #[derive(Component)]
/// struct Velocity { dx: f32, dy: f32 }
///
/// // Generate the world struct
/// define_world!(GameWorld {
///     Position,
///     Velocity,
/// });
///
/// // Usage
/// let mut world = GameWorld::new();
/// let player = world.spawn_entity();
///
/// world.add_component(player, Position { x: 0.0, y: 0.0 });
/// world.add_component(player, Velocity { dx: 1.0, dy: 0.0 });
///
/// // Update position based on velocity
/// if let (Some(pos), Some(vel)) = (
///     world.get_component_mut::<Position>(player),
///     world.get_component::<Velocity>(player)
/// ) {
///     pos.x += vel.dx;
///     pos.y += vel.dy;
/// }
/// ```
///
/// # Implementation Details
///
/// - Each component gets its own `VecStorage<Component>` field
/// - Storage fields are named using the component name in snake_case with "_storage" suffix
/// - All storage operations are type-safe through the `GetStorage` trait
/// - Entity IDs are simple incrementing integers starting from 0
#[proc_macro]
pub fn define_world(input: TokenStream) -> TokenStream {
    world::define_world_impl(input)
}
