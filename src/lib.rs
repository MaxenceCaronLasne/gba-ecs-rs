struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    dx: f32,
    dy: f32,
}

struct Health {
    value: i32,
}

struct World {
    position_storage: VecStorage<Position>,
    velocity_storage: VecStorage<Velocity>,
    health_storage: VecStorage<Health>,
    entity_count: usize,
}

trait GetStorage<C: Component> {
    type Storage: ComponentStorage<C>;
    fn get_storage(&self) -> &Self::Storage;
    fn get_storage_mut(&mut self) -> &mut Self::Storage;
}

impl GetStorage<Position> for World {
    type Storage = VecStorage<Position>;

    fn get_storage(&self) -> &Self::Storage {
        &self.position_storage
    }

    fn get_storage_mut(&mut self) -> &mut Self::Storage {
        &mut self.position_storage
    }
}

impl GetStorage<Velocity> for World {
    type Storage = VecStorage<Velocity>;

    fn get_storage(&self) -> &Self::Storage {
        &self.velocity_storage
    }

    fn get_storage_mut(&mut self) -> &mut Self::Storage {
        &mut self.velocity_storage
    }
}

impl GetStorage<Health> for World {
    type Storage = VecStorage<Health>;

    fn get_storage(&self) -> &Self::Storage {
        &self.health_storage
    }

    fn get_storage_mut(&mut self) -> &mut Self::Storage {
        &mut self.health_storage
    }
}

trait Component: 'static {}

trait ComponentStorage<C: Component> {
    fn new() -> Self;
    fn get(&self, entity: usize) -> Option<&C>;
    fn get_mut(&mut self, entity: usize) -> Option<&mut C>;
    fn insert(&mut self, entity: usize, component: C);
    fn remove(&mut self, entity: usize) -> Option<C>;
    fn ensure_capacity(&mut self, entity: usize);
}

struct VecStorage<C: Component> {
    components: Vec<Option<C>>,
}

impl<C: Component> ComponentStorage<C> for VecStorage<C> {
    fn new() -> Self {
        VecStorage {
            components: Vec::new(),
        }
    }

    fn get(&self, entity: usize) -> Option<&C> {
        self.components.get(entity)?.as_ref()
    }

    fn get_mut(&mut self, entity: usize) -> Option<&mut C> {
        self.components.get_mut(entity)?.as_mut()
    }

    fn insert(&mut self, entity: usize, component: C) {
        self.ensure_capacity(entity);
        self.components[entity] = Some(component);
    }

    fn remove(&mut self, entity: usize) -> Option<C> {
        if entity < self.components.len() {
            self.components[entity].take()
        } else {
            None
        }
    }

    fn ensure_capacity(&mut self, entity: usize) {
        if entity >= self.components.len() {
            self.components.resize_with(entity + 1, || None);
        }
    }
}

impl Component for Position {}
impl Component for Velocity {}
impl Component for Health {}

impl World {
    fn new() -> Self {
        World {
            position_storage: VecStorage::new(),
            velocity_storage: VecStorage::new(),
            health_storage: VecStorage::new(),
            entity_count: 0,
        }
    }

    fn spawn_entity(&mut self) -> usize {
        let entity_id = self.entity_count;
        self.entity_count += 1;
        entity_id
    }

    fn add_component<C: Component>(&mut self, entity: usize, component: C)
    where
        Self: GetStorage<C>,
    {
        self.get_storage_mut().insert(entity, component);
    }

    fn remove_component<C: Component>(&mut self, entity: usize) -> Option<C>
    where
        Self: GetStorage<C>,
    {
        self.get_storage_mut().remove(entity)
    }

    fn get_component<C: Component>(&self, entity: usize) -> Option<&C>
    where
        Self: GetStorage<C>,
    {
        self.get_storage().get(entity)
    }

    fn get_component_mut<C: Component>(&mut self, entity: usize) -> Option<&mut C>
    where
        Self: GetStorage<C>,
    {
        self.get_storage_mut().get_mut(entity)
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut world = World::new();
        let entity = world.spawn_entity();
        world.add_component(entity, Position { x: 0.0, y: 0.0 });

        let pos = world.get_component::<Position>(entity).unwrap();
        assert_eq!(pos.x, 0.0);
    }

    #[test]
    fn test_multiple_components() {
        let mut world = World::new();
        let entity = world.spawn_entity();

        // Test compile-time dispatch for different component types
        world.add_component(entity, Position { x: 1.0, y: 2.0 });
        world.add_component(entity, Velocity { dx: 0.5, dy: -0.5 });
        world.add_component(entity, Health { value: 100 });

        // Test retrieval
        let pos = world.get_component::<Position>(entity).unwrap();
        let vel = world.get_component::<Velocity>(entity).unwrap();
        let health = world.get_component::<Health>(entity).unwrap();

        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(vel.dx, 0.5);
        assert_eq!(vel.dy, -0.5);
        assert_eq!(health.value, 100);

        // Test mutable access
        {
            let pos_mut = world.get_component_mut::<Position>(entity).unwrap();
            pos_mut.x = 10.0;
        }

        let pos = world.get_component::<Position>(entity).unwrap();
        assert_eq!(pos.x, 10.0);

        // Test removal
        let removed_health = world.remove_component::<Health>(entity);
        assert_eq!(removed_health.unwrap().value, 100);
        assert!(world.get_component::<Health>(entity).is_none());
    }
}
