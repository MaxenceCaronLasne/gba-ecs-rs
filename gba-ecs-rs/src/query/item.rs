use crate::storage::{ComponentStorage, GetStorage};
use crate::{Component, Entity};

pub trait ComponentQuery<'w, W> {
    type Item;

    fn get_item(world: &'w W, entity: Entity) -> Option<Self::Item>;
    fn get_item_mut(world: &'w mut W, entity: Entity) -> Option<Self::Item>;
}

// Implementation for single immutable reference
impl<'w, T: Component, W: GetStorage<T>> ComponentQuery<'w, W> for &T {
    type Item = &'w T;

    fn get_item(world: &'w W, entity: Entity) -> Option<Self::Item> {
        world.get_storage().get(entity)
    }

    fn get_item_mut(world: &'w mut W, entity: Entity) -> Option<Self::Item> {
        world.get_storage().get(entity)
    }
}

// Implementation for single mutable reference
impl<'w, T: Component, W: GetStorage<T>> ComponentQuery<'w, W> for &mut T {
    type Item = &'w mut T;

    fn get_item(_world: &'w W, _entity: Entity) -> Option<Self::Item> {
        // Can't get mutable reference from immutable world
        None
    }

    fn get_item_mut(world: &'w mut W, entity: Entity) -> Option<Self::Item> {
        world.get_storage_mut().get_mut(entity)
    }
}

// Macro to implement ComponentQuery for tuples
macro_rules! impl_component_query_tuple {
    ($($T:ident),*) => {
        impl<'w, W, $($T),*> ComponentQuery<'w, W> for ($($T,)*)
        where
            $($T: ComponentQuery<'w, W>,)*
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

    };
}

// Implement for tuples of 1-4 components
impl_component_query_tuple!(A);
impl_component_query_tuple!(A, B);
impl_component_query_tuple!(A, B, C);
impl_component_query_tuple!(A, B, C, D);

