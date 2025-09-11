use crate::storage::{ComponentStorage, GetStorage};
use crate::{Component, Entity};
use crate::query::filter::Filter;

pub trait QueryItem<'w, W> {
    type Item;

    fn get_item(world: &'w W, entity: Entity) -> Option<Self::Item>;
    fn get_item_mut(world: &'w mut W, entity: Entity) -> Option<Self::Item>;
}

pub trait QueryItemWithFilter<'w, W, F> {
    type Item;

    fn get_item_filtered(world: &'w W, entity: Entity, filter: &F) -> Option<Self::Item>;
    fn get_item_mut_filtered(world: &'w mut W, entity: Entity, filter: &F) -> Option<Self::Item>;
}

// Implementation for single immutable reference
impl<'w, T: Component, W: GetStorage<T>> QueryItem<'w, W> for &T {
    type Item = &'w T;

    fn get_item(world: &'w W, entity: Entity) -> Option<Self::Item> {
        world.get_storage().get(entity)
    }

    fn get_item_mut(world: &'w mut W, entity: Entity) -> Option<Self::Item> {
        world.get_storage().get(entity)
    }
}

// Implementation for single mutable reference
impl<'w, T: Component, W: GetStorage<T>> QueryItem<'w, W> for &mut T {
    type Item = &'w mut T;

    fn get_item(_world: &'w W, _entity: Entity) -> Option<Self::Item> {
        // Can't get mutable reference from immutable world
        None
    }

    fn get_item_mut(world: &'w mut W, entity: Entity) -> Option<Self::Item> {
        world.get_storage_mut().get_mut(entity)
    }
}

// QueryItemWithFilter implementations for single components
impl<'w, T: Component, W: GetStorage<T>, F: Filter<W>> QueryItemWithFilter<'w, W, F> for &T {
    type Item = &'w T;

    fn get_item_filtered(world: &'w W, entity: Entity, filter: &F) -> Option<Self::Item> {
        if filter.matches(world, entity.index) {
            world.get_storage().get(entity)
        } else {
            None
        }
    }

    fn get_item_mut_filtered(world: &'w mut W, entity: Entity, filter: &F) -> Option<Self::Item> {
        if filter.matches(world, entity.index) {
            world.get_storage().get(entity)
        } else {
            None
        }
    }
}

impl<'w, T: Component, W: GetStorage<T>, F: Filter<W>> QueryItemWithFilter<'w, W, F> for &mut T {
    type Item = &'w mut T;

    fn get_item_filtered(_world: &'w W, _entity: Entity, _filter: &F) -> Option<Self::Item> {
        // Can't get mutable reference from immutable world
        None
    }

    fn get_item_mut_filtered(world: &'w mut W, entity: Entity, filter: &F) -> Option<Self::Item> {
        if filter.matches(world, entity.index) {
            world.get_storage_mut().get_mut(entity)
        } else {
            None
        }
    }
}

// Macro to implement QueryItem for tuples
macro_rules! impl_query_item_tuple {
    ($($T:ident),*) => {
        impl<'w, W, $($T),*> QueryItem<'w, W> for ($($T,)*)
        where
            $($T: QueryItem<'w, W>,)*
        {
            type Item = ($($T::Item,)*);

            fn get_item(world: &'w W, entity: Entity) -> Option<Self::Item> {
                Some(($($T::get_item(world, entity)?,)*))
            }

            fn get_item_mut(world: &'w mut W, entity: Entity) -> Option<Self::Item> {
                // This is tricky - we need unsafe code to get multiple mutable references
                // For now, let's implement a simpler version that works for mixed queries
                unsafe {
                    let world_ptr = world as *mut W;
                    Some(($(
                        $T::get_item_mut(&mut *world_ptr, entity)?,
                    )*))
                }
            }
        }

        impl<'w, W, F, $($T),*> QueryItemWithFilter<'w, W, F> for ($($T,)*)
        where
            F: Filter<W>,
            $($T: QueryItemWithFilter<'w, W, F>,)*
        {
            type Item = ($($T::Item,)*);

            fn get_item_filtered(world: &'w W, entity: Entity, filter: &F) -> Option<Self::Item> {
                Some(($($T::get_item_filtered(world, entity, filter)?,)*))
            }

            fn get_item_mut_filtered(world: &'w mut W, entity: Entity, filter: &F) -> Option<Self::Item> {
                unsafe {
                    let world_ptr = world as *mut W;
                    Some(($(
                        $T::get_item_mut_filtered(&mut *world_ptr, entity, filter)?,
                    )*))
                }
            }
        }
    };
}

// Implement for tuples of 1-4 components
impl_query_item_tuple!(A);
impl_query_item_tuple!(A, B);
impl_query_item_tuple!(A, B, C);
impl_query_item_tuple!(A, B, C, D);

