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
}
