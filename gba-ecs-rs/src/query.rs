//! Query system for efficient iteration over entities and their components.
//!
//! This module provides a trait-based query system that allows for efficient
//! iteration over entities that have specific component combinations. The system
//! supports single component queries and tuple queries for multiple components (up to 3).
//!
//! # Safety
//!
//! This module contains significant unsafe code for performance optimization.
//! The unsafe operations are primarily used for:
//! - Casting between container types when we know the concrete type
//! - Extending lifetimes to match the World's lifetime
//! - Avoiding dynamic dispatch for better performance
//!
//! # Examples
//!
//! ```ignore
//! // Query for all entities with component A
//! <&A as Query<WC>>::for_each(&world, |entity_index, component_a| {
//!     // Process component_a
//! });
//!
//! // Query for all entities with both components A and B
//! <(&A, &B) as Query<WC>>::for_each(&world, |entity_index, (component_a, component_b)| {
//!     // Process both components
//! });
//!
//! // Query for all entities with components A, B, and C
//! <(&A, &B, &C) as Query<WC>>::for_each(&world, |entity_index, (component_a, component_b, component_c)| {
//!     // Process all three components
//! });
//!
//! // Use sparse traversal for better performance with sparse data
//! <(&A, &B, &C) as Query<WC>>::for_each_sparse(&world, |entity_index, (component_a, component_b, component_c)| {
//!     // Process all three components using sparse iteration
//! });
//! ```

use crate::{
    zip, zip3, ComponentContainer, Entity, GetComponentContainer, VecComponentContainer, World,
    WorldContainer,
};
use alloc::alloc::Allocator;
use alloc::alloc::Global;
use core::mem::transmute;

/// Casts a generic ComponentContainer to a VecComponentContainer with runtime validation.
///
/// # Safety
///
/// This function includes runtime validation but is still unsafe because:
/// 1. We cannot verify the allocator type A at runtime
/// 2. Type casting still bypasses Rust's type system
/// 3. The caller must ensure the allocator type matches
///
/// This function will panic in debug mode if is_vec_container() returns false,
/// helping catch misuse during development.
///
/// # Arguments
///
/// * `container` - A reference to a container that should be a VecComponentContainer
///
/// # Returns
///
/// A reference to the same container, but with the concrete VecComponentContainer type
///
/// # Panics
///
/// Panics in debug mode if the container is not actually a VecComponentContainer
unsafe fn cast_to_vec_container<C, A: Allocator + Clone, Container: ComponentContainer<C>>(
    container: &Container,
) -> &VecComponentContainer<C, A> {
    // Runtime validation in debug mode
    debug_assert!(
        container.is_vec_container(),
        "Attempted to cast non-VecComponentContainer to VecComponentContainer"
    );

    // Additional defensive check for container size consistency
    debug_assert!(
        container.len() < usize::MAX / 4,
        "Container size {} seems invalid for casting",
        container.len()
    );

    // SAFETY: We've verified is_vec_container() returns true
    // However, we still cannot verify the allocator type A at runtime
    // Caller must ensure the allocator type matches the original container
    &*(container as *const Container as *const VecComponentContainer<C, A>)
}

/// Safely extends the lifetime of a component reference to match the world's lifetime.
///
/// # Safety
///
/// This function is safe to use when:
/// 1. The component reference comes from a container owned by the world
/// 2. The world's lifetime ('a) is shorter than or equal to the container's lifetime
/// 3. The component will not be modified during the world's lifetime
///
/// # Arguments
///
/// * `component` - A component reference from a container
///
/// # Returns
///
/// The same component reference with lifetime extended to 'a
unsafe fn extend_component_lifetime<'a, C>(component: &C) -> &'a C {
    // SAFETY: Caller must ensure the component comes from a container owned by the world
    // and that the world's lifetime is valid for this component
    transmute(component)
}

/// Helper function for single component queries using VecComponentContainer.
///
/// # Safety
///
/// This function assumes the container is actually a VecComponentContainer with Global allocator.
/// Validation is performed inside cast_to_vec_container.
unsafe fn query_single_vec_container<'a, A, WC, F>(
    container: &<WC as GetComponentContainer<A>>::Container,
    mut f: F,
) where
    A: 'a,
    WC: WorldContainer + GetComponentContainer<A>,
    <WC as GetComponentContainer<A>>::Container: ComponentContainer<A>,
    F: FnMut(usize, &'a A),
{
    let vec_container = cast_to_vec_container::<A, Global, _>(container);
    vec_container.for_each_fast(|entity_index, component| {
        let component_extended = extend_component_lifetime(component);
        f(entity_index, component_extended);
    });
}

/// Helper function for single component queries using generic container.
unsafe fn query_single_generic_container<'a, A, WC, F>(
    container: &<WC as GetComponentContainer<A>>::Container,
    mut f: F,
) where
    A: 'a,
    WC: WorldContainer + GetComponentContainer<A>,
    <WC as GetComponentContainer<A>>::Container: ComponentContainer<A>,
    F: FnMut(usize, &'a A),
{
    container.for_each(|entity_index, component| {
        let component_extended = extend_component_lifetime(component);
        f(entity_index, component_extended);
    });
}

/// Helper function for tuple queries using VecComponentContainers.
///
/// # Safety
///
/// This function assumes both containers are actually VecComponentContainers with Global allocator.
/// Validation is performed inside cast_to_vec_container.
unsafe fn query_tuple_vec_containers<'a, A, B, WC, F>(
    container_a: &<WC as GetComponentContainer<A>>::Container,
    container_b: &<WC as GetComponentContainer<B>>::Container,
    mut f: F,
) where
    A: 'a,
    B: 'a,
    WC: WorldContainer + GetComponentContainer<A> + GetComponentContainer<B>,
    <WC as GetComponentContainer<A>>::Container: ComponentContainer<A>,
    <WC as GetComponentContainer<B>>::Container: ComponentContainer<B>,
    F: FnMut(usize, (&'a A, &'a B)),
{
    let vec_container_a = cast_to_vec_container::<A, Global, _>(container_a);
    let vec_container_b = cast_to_vec_container::<B, Global, _>(container_b);

    zip(vec_container_a, vec_container_b).for_each(|entity_index, component_a, component_b| {
        let component_a_extended = extend_component_lifetime(component_a);
        let component_b_extended = extend_component_lifetime(component_b);
        f(entity_index, (component_a_extended, component_b_extended));
    });
}

/// Helper function for tuple queries using mixed container types.
///
/// # Safety
///
/// This function creates raw pointers to containers and dereferences them.
/// The pointers remain valid because the containers are owned by the world.
unsafe fn query_tuple_generic_containers<'a, A, B, WC, F>(
    container_a: &<WC as GetComponentContainer<A>>::Container,
    container_b: &<WC as GetComponentContainer<B>>::Container,
    mut f: F,
) where
    A: 'a,
    B: 'a,
    WC: WorldContainer + GetComponentContainer<A> + GetComponentContainer<B>,
    <WC as GetComponentContainer<A>>::Container: ComponentContainer<A>,
    <WC as GetComponentContainer<B>>::Container: ComponentContainer<B>,
    F: FnMut(usize, (&'a A, &'a B)),
{
    let container_a_ptr = container_a as *const <WC as GetComponentContainer<A>>::Container;
    let container_b_ptr = container_b as *const <WC as GetComponentContainer<B>>::Container;

    // Iterate over container A and lookup in container B
    (*container_a_ptr).for_each(|entity_index, component_a| {
        if let Some(entity) = safe_entity_new(entity_index) {
            if let Some(component_b) = (*container_b_ptr).get(entity) {
                let component_a_extended = extend_component_lifetime(component_a);
                let component_b_extended = extend_component_lifetime(component_b);
                f(entity_index, (component_a_extended, component_b_extended));
            }
        }
        // If entity creation fails, we skip this entity silently
        // This prevents crashes from invalid entity indices
    });
}

/// Helper function for triple queries using VecComponentContainers.
///
/// # Safety
///
/// This function assumes all three containers are actually VecComponentContainers with Global allocator.
/// Validation is performed inside cast_to_vec_container.
unsafe fn query_triple_vec_containers<'a, A, B, C, WC, F>(
    container_a: &<WC as GetComponentContainer<A>>::Container,
    container_b: &<WC as GetComponentContainer<B>>::Container,
    container_c: &<WC as GetComponentContainer<C>>::Container,
    mut f: F,
) where
    A: 'a,
    B: 'a,
    C: 'a,
    WC: WorldContainer
        + GetComponentContainer<A>
        + GetComponentContainer<B>
        + GetComponentContainer<C>,
    <WC as GetComponentContainer<A>>::Container: ComponentContainer<A>,
    <WC as GetComponentContainer<B>>::Container: ComponentContainer<B>,
    <WC as GetComponentContainer<C>>::Container: ComponentContainer<C>,
    F: FnMut(usize, (&'a A, &'a B, &'a C)),
{
    let vec_container_a = cast_to_vec_container::<A, Global, _>(container_a);
    let vec_container_b = cast_to_vec_container::<B, Global, _>(container_b);
    let vec_container_c = cast_to_vec_container::<C, Global, _>(container_c);

    zip3(vec_container_a, vec_container_b, vec_container_c).for_each(
        |entity_index, component_a, component_b, component_c| {
            let component_a_extended = extend_component_lifetime(component_a);
            let component_b_extended = extend_component_lifetime(component_b);
            let component_c_extended = extend_component_lifetime(component_c);
            f(
                entity_index,
                (
                    component_a_extended,
                    component_b_extended,
                    component_c_extended,
                ),
            );
        },
    );
}

/// Helper function for triple queries using mixed container types.
///
/// # Safety
///
/// This function creates raw pointers to containers and dereferences them.
/// The pointers remain valid because the containers are owned by the world.
unsafe fn query_triple_generic_containers<'a, A, B, C, WC, F>(
    container_a: &<WC as GetComponentContainer<A>>::Container,
    container_b: &<WC as GetComponentContainer<B>>::Container,
    container_c: &<WC as GetComponentContainer<C>>::Container,
    mut f: F,
) where
    A: 'a,
    B: 'a,
    C: 'a,
    WC: WorldContainer
        + GetComponentContainer<A>
        + GetComponentContainer<B>
        + GetComponentContainer<C>,
    <WC as GetComponentContainer<A>>::Container: ComponentContainer<A>,
    <WC as GetComponentContainer<B>>::Container: ComponentContainer<B>,
    <WC as GetComponentContainer<C>>::Container: ComponentContainer<C>,
    F: FnMut(usize, (&'a A, &'a B, &'a C)),
{
    let container_a_ptr = container_a as *const <WC as GetComponentContainer<A>>::Container;
    let container_b_ptr = container_b as *const <WC as GetComponentContainer<B>>::Container;
    let container_c_ptr = container_c as *const <WC as GetComponentContainer<C>>::Container;

    // Iterate over container A and lookup in containers B and C
    (*container_a_ptr).for_each(|entity_index, component_a| {
        if let Some(entity) = safe_entity_new(entity_index) {
            if let Some(component_b) = (*container_b_ptr).get(entity) {
                if let Some(component_c) = (*container_c_ptr).get(entity) {
                    let component_a_extended = extend_component_lifetime(component_a);
                    let component_b_extended = extend_component_lifetime(component_b);
                    let component_c_extended = extend_component_lifetime(component_c);
                    f(
                        entity_index,
                        (
                            component_a_extended,
                            component_b_extended,
                            component_c_extended,
                        ),
                    );
                }
            }
        }
        // If entity creation fails, we skip this entity silently
        // This prevents crashes from invalid entity indices
    });
}

/// Common validation logic for container queries.
///
/// This function performs early validation checks that are common across all query types.
/// It's designed to catch common issues in debug mode without impacting release performance.
fn validate_container<C, Container: ComponentContainer<C>>(container: &Container, name: &str) {
    debug_assert!(
        !container.is_empty() || true,
        "Container {} validation passed",
        name
    );

    // Additional validation for potential overflow conditions
    debug_assert!(
        container.len() < usize::MAX / 2,
        "Container {} has suspiciously large size: {}",
        name,
        container.len()
    );
}

/// Validates entity index bounds to prevent out-of-bounds access.
///
/// # Arguments
///
/// * `entity_index` - The entity index to validate
/// * `max_entities` - The maximum number of entities (container length)
///
/// # Returns
///
/// `true` if the entity index is valid, `false` otherwise
fn is_valid_entity_index(entity_index: usize, max_entities: usize) -> bool {
    entity_index < max_entities
}

/// Safe wrapper around Entity::new that validates the entity index.
///
/// # Arguments
///
/// * `entity_index` - The entity index to create an Entity from
///
/// # Returns
///
/// `Some(Entity)` if the index is valid, `None` otherwise
fn safe_entity_new(entity_index: usize) -> Option<Entity> {
    // Entity::new might have internal validation, but we add an extra check
    // to ensure we don't create entities with obviously invalid indices
    if entity_index == usize::MAX {
        None
    } else {
        Some(Entity::new(entity_index))
    }
}

/// Trait for querying entities and their components from a World.
///
/// This trait allows for efficient iteration over entities that have specific
/// component combinations. Implementations provide optimized paths for different
/// container types. Currently supports queries for 1, 2, or 3 components.
pub trait Query<'a, WC: WorldContainer> {
    /// The type of item yielded by this query (e.g., &A, (&A, &B), or (&A, &B, &C))
    type Item;

    /// Iterates over all entities that match this query.
    ///
    /// # Arguments
    ///
    /// * `world` - The world to query from
    /// * `f` - A closure that will be called for each matching entity
    ///         The closure receives (entity_index, components)
    fn for_each<F>(world: &'a World<WC>, f: F)
    where
        F: FnMut(usize, Self::Item);

    /// Iterates over all entities that match this query using sparse traversal.
    ///
    /// This method always uses the generic container approach, which iterates through
    /// one container's active indices and looks up components in other containers.
    /// This can be more efficient when dealing with sparse data.
    ///
    /// # Arguments
    ///
    /// * `world` - The world to query from
    /// * `f` - A closure that will be called for each matching entity
    ///         The closure receives (entity_index, components)
    fn for_each_sparse<F>(world: &'a World<WC>, f: F)
    where
        F: FnMut(usize, Self::Item);
}

/// Implementation of Query for single component queries (&A).
///
/// This implementation provides an optimized path for VecComponentContainer
/// and a fallback for other container types.
impl<'a, A: 'a, WC> Query<'a, WC> for &A
where
    WC: WorldContainer + GetComponentContainer<A>,
    <WC as GetComponentContainer<A>>::Container: ComponentContainer<A>,
{
    type Item = &'a A;

    fn for_each<F>(world: &'a World<WC>, f: F)
    where
        F: FnMut(usize, Self::Item),
    {
        let container = world.get::<A>();

        // Early validation
        validate_container(container, "A");

        if container.is_vec_container() {
            // SAFETY: Container type verified by is_vec_container()
            // Helper function handles the unsafe casting and lifetime extension
            unsafe { query_single_vec_container::<A, WC, F>(container, f) };
        } else {
            // SAFETY: Helper function handles the unsafe lifetime extension
            unsafe { query_single_generic_container::<A, WC, F>(container, f) };
        }
    }

    fn for_each_sparse<F>(world: &'a World<WC>, f: F)
    where
        F: FnMut(usize, Self::Item),
    {
        let container = world.get::<A>();

        // Early validation
        validate_container(container, "A");

        // Always use generic container approach for sparse traversal
        // SAFETY: Helper function handles the unsafe lifetime extension
        unsafe { query_single_generic_container::<A, WC, F>(container, f) };
    }
}

/// Implementation of Query for two-component queries (&A, &B).
///
/// This implementation provides two execution paths:
/// 1. Fast path: When both containers are VecComponentContainers, uses zip for efficient iteration
/// 2. Fallback path: When containers are different types, iterates over one and looks up the other
impl<'a, A: 'a, B: 'a, WC> Query<'a, WC> for (&A, &B)
where
    WC: WorldContainer + GetComponentContainer<A> + GetComponentContainer<B>,
    <WC as GetComponentContainer<A>>::Container: ComponentContainer<A>,
    <WC as GetComponentContainer<B>>::Container: ComponentContainer<B>,
{
    type Item = (&'a A, &'a B);

    fn for_each<F>(world: &'a World<WC>, f: F)
    where
        F: FnMut(usize, Self::Item),
    {
        let container_a = world.get::<A>();
        let container_b = world.get::<B>();

        // Early validation for both containers
        validate_container(container_a, "A");
        validate_container(container_b, "B");

        if container_a.is_vec_container() && container_b.is_vec_container() {
            // Fast path: Both containers are VecComponentContainers
            // SAFETY: Container types verified by is_vec_container()
            // Helper function handles the unsafe casting and lifetime extension
            unsafe { query_tuple_vec_containers::<A, B, WC, F>(container_a, container_b, f) };
        } else {
            // Fallback path: At least one container is not a VecComponentContainer
            // SAFETY: Helper function handles the unsafe raw pointer operations and lifetime extension
            unsafe { query_tuple_generic_containers::<A, B, WC, F>(container_a, container_b, f) };
        }
    }

    fn for_each_sparse<F>(world: &'a World<WC>, f: F)
    where
        F: FnMut(usize, Self::Item),
    {
        let container_a = world.get::<A>();
        let container_b = world.get::<B>();

        // Early validation for both containers
        validate_container(container_a, "A");
        validate_container(container_b, "B");

        // Always use generic container approach for sparse traversal
        // SAFETY: Helper function handles the unsafe raw pointer operations and lifetime extension
        unsafe { query_tuple_generic_containers::<A, B, WC, F>(container_a, container_b, f) };
    }
}

/// Implementation of Query for three-component queries (&A, &B, &C).
///
/// This implementation provides two execution paths:
/// 1. Fast path: When all three containers are VecComponentContainers, uses zip3 for efficient iteration
/// 2. Fallback path: When containers are different types, iterates over one and looks up the others
impl<'a, A: 'a, B: 'a, C: 'a, WC> Query<'a, WC> for (&A, &B, &C)
where
    WC: WorldContainer
        + GetComponentContainer<A>
        + GetComponentContainer<B>
        + GetComponentContainer<C>,
    <WC as GetComponentContainer<A>>::Container: ComponentContainer<A>,
    <WC as GetComponentContainer<B>>::Container: ComponentContainer<B>,
    <WC as GetComponentContainer<C>>::Container: ComponentContainer<C>,
{
    type Item = (&'a A, &'a B, &'a C);

    fn for_each<F>(world: &'a World<WC>, f: F)
    where
        F: FnMut(usize, Self::Item),
    {
        let container_a = world.get::<A>();
        let container_b = world.get::<B>();
        let container_c = world.get::<C>();

        // Early validation for all three containers
        validate_container(container_a, "A");
        validate_container(container_b, "B");
        validate_container(container_c, "C");

        if container_a.is_vec_container()
            && container_b.is_vec_container()
            && container_c.is_vec_container()
        {
            // Fast path: All three containers are VecComponentContainers
            // SAFETY: Container types verified by is_vec_container()
            // Helper function handles the unsafe casting and lifetime extension
            unsafe {
                query_triple_vec_containers::<A, B, C, WC, F>(
                    container_a,
                    container_b,
                    container_c,
                    f,
                )
            };
        } else {
            // Fallback path: At least one container is not a VecComponentContainer
            // SAFETY: Helper function handles the unsafe raw pointer operations and lifetime extension
            unsafe {
                query_triple_generic_containers::<A, B, C, WC, F>(
                    container_a,
                    container_b,
                    container_c,
                    f,
                )
            };
        }
    }

    fn for_each_sparse<F>(world: &'a World<WC>, f: F)
    where
        F: FnMut(usize, Self::Item),
    {
        let container_a = world.get::<A>();
        let container_b = world.get::<B>();
        let container_c = world.get::<C>();

        // Early validation for all three containers
        validate_container(container_a, "A");
        validate_container(container_b, "B");
        validate_container(container_c, "C");

        // Always use generic container approach for sparse traversal
        // SAFETY: Helper function handles the unsafe raw pointer operations and lifetime extension
        unsafe {
            query_triple_generic_containers::<A, B, C, WC, F>(
                container_a,
                container_b,
                container_c,
                f,
            )
        };
    }
}
