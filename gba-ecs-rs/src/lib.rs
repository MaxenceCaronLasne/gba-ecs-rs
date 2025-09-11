#![no_std]

extern crate alloc;
use alloc::vec::Vec;

// Re-export procedural macros from the macro crate
pub use gba_ecs_macros::*;

#[derive(Clone, Copy, Debug)]
pub struct Entity {
    pub index: usize,
}

pub trait GetStorage<C: Component> {
    type Storage: ComponentStorage<C>;
    fn get_storage(&self) -> &Self::Storage;
    fn get_storage_mut(&mut self) -> &mut Self::Storage;
}

pub trait Component: 'static {}

// Query system traits
pub trait QueryItem<'w, W> {
    type Item;

    fn get_item(world: &'w W, entity: Entity) -> Option<Self::Item>;
    fn get_item_mut(world: &'w mut W, entity: Entity) -> Option<Self::Item>;
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
    };
}

// Implement for tuples of 1-4 components
impl_query_item_tuple!(A);
impl_query_item_tuple!(A, B);
impl_query_item_tuple!(A, B, C);
impl_query_item_tuple!(A, B, C, D);

// Query iterator
pub struct QueryIterator<'w, Q, W> {
    world: &'w mut W,
    current_entity: usize,
    max_entity: usize,
    _phantom: core::marker::PhantomData<Q>,
}

impl<'w, Q, W> QueryIterator<'w, Q, W> {
    pub fn new(world: &'w mut W, max_entity: usize) -> Self {
        QueryIterator {
            world,
            current_entity: 0,
            max_entity,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<'w, Q: QueryItem<'w, W>, W> Iterator for QueryIterator<'w, Q, W> {
    type Item = Q::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_entity < self.max_entity {
            let entity = Entity {
                index: self.current_entity,
            };
            self.current_entity += 1;

            unsafe {
                // We need unsafe here to get around the borrow checker
                // This is safe because we know each component storage is independent
                let world_ptr = self.world as *mut W;
                if let Some(item) = Q::get_item_mut(&mut *world_ptr, entity) {
                    return Some(item);
                }
            }
        }
        None
    }
}

pub trait ComponentStorage<C: Component> {
    fn new() -> Self;
    fn get(&self, entity: Entity) -> Option<&C>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut C>;
    fn insert(&mut self, entity: Entity, component: C);
    fn remove(&mut self, entity: Entity) -> Option<C>;
    fn ensure_capacity(&mut self, entity: Entity);
}

pub struct VecStorage<C: Component> {
    components: Vec<Option<C>>,
}

impl<C: Component> ComponentStorage<C> for VecStorage<C> {
    fn new() -> Self {
        VecStorage {
            components: Vec::new(),
        }
    }

    fn get(&self, entity: Entity) -> Option<&C> {
        self.components.get(entity.index)?.as_ref()
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.components.get_mut(entity.index)?.as_mut()
    }

    fn insert(&mut self, entity: Entity, component: C) {
        self.ensure_capacity(entity);
        self.components[entity.index] = Some(component);
    }

    fn remove(&mut self, entity: Entity) -> Option<C> {
        if entity.index < self.components.len() {
            self.components[entity.index].take()
        } else {
            None
        }
    }

    fn ensure_capacity(&mut self, entity: Entity) {
        if entity.index >= self.components.len() {
            self.components.resize_with(entity.index + 1, || None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as gba_ecs_rs; // Alias for the macro to find the crate
    use gba_ecs_macros::{define_world, Component};

    #[derive(Component)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Component)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }

    #[derive(Component)]
    struct Health {
        value: i32,
    }

    define_world!(World {
        Position,
        Velocity,
        Health,
    });

    #[test]
    fn it_works() {
        let mut world = World::new();
        let entity = world.spawn_entity();
        world.add_component(entity, Position { x: 1.0, y: 0.0 });

        let pos = world.get_component::<Position>(entity).unwrap();
        assert_eq!(pos.x, 1.0);
    }

    #[test]
    fn test_multiple_components() {
        let mut world = World::new();
        let entity = world.spawn_entity();

        world.add_component(entity, Position { x: 1.0, y: 2.0 });
        world.add_component(entity, Velocity { dx: 0.5, dy: -0.5 });
        world.add_component(entity, Health { value: 100 });

        let pos = world.get_component::<Position>(entity).unwrap();
        let vel = world.get_component::<Velocity>(entity).unwrap();
        let health = world.get_component::<Health>(entity).unwrap();

        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(vel.dx, 0.5);
        assert_eq!(vel.dy, -0.5);
        assert_eq!(health.value, 100);

        {
            let pos_mut = world.get_component_mut::<Position>(entity).unwrap();
            pos_mut.x = 10.0;
        }

        let pos = world.get_component::<Position>(entity).unwrap();
        assert_eq!(pos.x, 10.0);

        let removed_health = world.remove_component::<Health>(entity);
        assert_eq!(removed_health.unwrap().value, 100);
        assert!(world.get_component::<Health>(entity).is_none());
    }

    #[test]
    fn test_single_component_query_readonly() {
        let mut world = World::new();

        // Create entities with Position components
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });

        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 3.0, y: 4.0 });

        // Entity without Position
        let _entity3 = world.spawn_entity();
        world.add_component(_entity3, Health { value: 50 });

        // Query all entities with Position
        let mut positions = Vec::new();
        for pos in world.query::<&Position>() {
            positions.push((pos.x, pos.y));
        }

        // Should find exactly 2 positions
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&(1.0, 2.0)));
        assert!(positions.contains(&(3.0, 4.0)));
    }

    #[test]
    fn test_single_component_query_mutable() {
        let mut world = World::new();

        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });

        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 3.0, y: 4.0 });

        // Modify all positions through query
        for pos in world.query::<&mut Position>() {
            pos.x += 10.0;
            pos.y += 20.0;
        }

        // Verify modifications
        let pos1 = world.get_component::<Position>(entity1).unwrap();
        assert_eq!(pos1.x, 11.0);
        assert_eq!(pos1.y, 22.0);

        let pos2 = world.get_component::<Position>(entity2).unwrap();
        assert_eq!(pos2.x, 13.0);
        assert_eq!(pos2.y, 24.0);
    }

    #[test]
    fn test_two_component_query() {
        let mut world = World::new();

        // Entity with Position and Velocity
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });
        world.add_component(entity1, Velocity { dx: 0.5, dy: -0.5 });

        // Entity with Position only
        let _entity2 = world.spawn_entity();
        world.add_component(_entity2, Position { x: 3.0, y: 4.0 });

        // Entity with Velocity only
        let _entity3 = world.spawn_entity();
        world.add_component(_entity3, Velocity { dx: 1.0, dy: 1.0 });

        // Query for entities with both Position and Velocity
        let mut results = Vec::new();
        for (pos, vel) in world.query::<(&Position, &Velocity)>() {
            results.push((pos.x, pos.y, vel.dx, vel.dy));
        }

        // Should only find entity1
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], (1.0, 2.0, 0.5, -0.5));
    }

    #[test]
    fn test_two_component_query_with_mutation() {
        let mut world = World::new();

        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 0.0, y: 0.0 });
        world.add_component(entity1, Velocity { dx: 1.0, dy: 2.0 });

        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 5.0, y: 5.0 });
        world.add_component(entity2, Velocity { dx: -1.0, dy: -2.0 });

        // Update positions based on velocity
        for (pos, vel) in world.query::<(&mut Position, &Velocity)>() {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }

        // Verify updates
        let pos1 = world.get_component::<Position>(entity1).unwrap();
        assert_eq!(pos1.x, 1.0);
        assert_eq!(pos1.y, 2.0);

        let pos2 = world.get_component::<Position>(entity2).unwrap();
        assert_eq!(pos2.x, 4.0);
        assert_eq!(pos2.y, 3.0);
    }

    #[test]
    fn test_three_component_query() {
        let mut world = World::new();

        // Entity with all three components
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });
        world.add_component(entity1, Velocity { dx: 0.5, dy: -0.5 });
        world.add_component(entity1, Health { value: 100 });

        // Entity missing Health
        let _entity2 = world.spawn_entity();
        world.add_component(_entity2, Position { x: 3.0, y: 4.0 });
        world.add_component(_entity2, Velocity { dx: 1.0, dy: 1.0 });

        // Query for entities with all three components
        let mut count = 0;
        for (pos, vel, health) in world.query::<(&mut Position, &Velocity, &Health)>() {
            count += 1;
            pos.x += vel.dx * (health.value as f32 / 100.0);
            pos.y += vel.dy * (health.value as f32 / 100.0);
        }

        // Should only find entity1
        assert_eq!(count, 1);

        // Verify the update occurred
        let pos1 = world.get_component::<Position>(entity1).unwrap();
        assert_eq!(pos1.x, 1.5); // 1.0 + 0.5 * 1.0
        assert_eq!(pos1.y, 1.5); // 2.0 + (-0.5) * 1.0
    }

    #[test]
    fn test_query_empty_results() {
        let mut world = World::new();

        // Create entities but none have both Position and Velocity
        let _entity1 = world.spawn_entity();
        world.add_component(_entity1, Position { x: 1.0, y: 2.0 });

        let _entity2 = world.spawn_entity();
        world.add_component(_entity2, Velocity { dx: 1.0, dy: 2.0 });

        // Query for entities with both components - should be empty
        let mut count = 0;
        for (_pos, _vel) in world.query::<(&Position, &Velocity)>() {
            count += 1;
        }
        assert_eq!(count, 0);

        // Query for non-existent component type should also be empty
        let mut health_count = 0;
        for _health in world.query::<&Health>() {
            health_count += 1;
        }
        assert_eq!(health_count, 0);
    }

    #[test]
    fn test_query_iteration_count() {
        let mut world = World::new();

        // Create multiple entities with Position
        for i in 0..5 {
            let entity = world.spawn_entity();
            world.add_component(
                entity,
                Position {
                    x: i as f32,
                    y: (i * 2) as f32,
                },
            );
        }

        // Create entities with Velocity (some overlap, some don't)
        for i in 0..3 {
            let entity = Entity { index: i };
            world.add_component(
                entity,
                Velocity {
                    dx: i as f32,
                    dy: i as f32,
                },
            );
        }

        // Count Position queries
        let position_count = world.query::<&Position>().count();
        assert_eq!(position_count, 5);

        // Count Velocity queries
        let velocity_count = world.query::<&Velocity>().count();
        assert_eq!(velocity_count, 3);

        // Count combined queries (should be entities 0, 1, 2)
        let combined_count = world.query::<(&Position, &Velocity)>().count();
        assert_eq!(combined_count, 3);
    }
}
