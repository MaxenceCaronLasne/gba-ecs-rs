#[cfg(test)]
mod test {
    use agb::{ExternalAllocator, InternalAllocator};
    use alloc::vec;
    use alloc::vec::Vec;
    use gba_ecs_rs::{ComponentContainer, Entity, VecComponentContainer, World};

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestPosition {
        x: i32,
        y: i32,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestVelocity {
        dx: i32,
        dy: i32,
    }

    struct MacroTestWorld {
        test_position: VecComponentContainer<TestPosition, ExternalAllocator>,
        test_velocity: VecComponentContainer<TestVelocity, InternalAllocator>,
    }

    impl gba_ecs_rs::WorldContainer for MacroTestWorld {
        fn new() -> Self {
            Self {
                test_position: VecComponentContainer::new_in(ExternalAllocator),
                test_velocity: VecComponentContainer::new_in(InternalAllocator),
            }
        }
        fn add_entity(&mut self, entity: Entity) {
            self.test_position.add_entity(entity);
            self.test_velocity.add_entity(entity);
        }
    }

    impl gba_ecs_rs::GetComponentContainer<TestPosition> for MacroTestWorld {
        type Container = VecComponentContainer<TestPosition, ExternalAllocator>;
        fn get_components(&self) -> &Self::Container {
            &self.test_position
        }
        fn get_components_mut(&mut self) -> &mut Self::Container {
            &mut self.test_position
        }
    }

    impl gba_ecs_rs::GetComponentContainer<TestVelocity> for MacroTestWorld {
        type Container = VecComponentContainer<TestVelocity, InternalAllocator>;
        fn get_components(&self) -> &Self::Container {
            &self.test_velocity
        }
        fn get_components_mut(&mut self) -> &mut Self::Container {
            &mut self.test_velocity
        }
    }

    #[test_case]
    fn test_world_macro(_agb: &mut agb::Gba) {
        let mut world = World::<MacroTestWorld>::new();

        let entity1 = world.spawn();
        let entity2 = world.spawn();

        world.add(entity1, TestPosition { x: 10, y: 20 });
        world.add(entity1, TestVelocity { dx: 1, dy: 2 });
        world.add(entity2, TestPosition { x: 30, y: 40 });

        let positions = world.get::<TestPosition>();
        let velocities = world.get::<TestVelocity>();

        let pos1 = positions.get(entity1).unwrap();
        assert_eq!(pos1.x, 10);
        assert_eq!(pos1.y, 20);

        let pos2 = positions.get(entity2).unwrap();
        assert_eq!(pos2.x, 30);
        assert_eq!(pos2.y, 40);

        let vel1 = velocities.get(entity1).unwrap();
        assert_eq!(vel1.dx, 1);
        assert_eq!(vel1.dy, 2);

        assert!(velocities.get(entity2).is_none());

        let positions_mut = world.get_mut::<TestPosition>();
        if let Some(pos) = positions_mut.get_mut(entity1) {
            pos.x += 5;
            pos.y += 10;
        }

        let positions = world.get::<TestPosition>();
        let pos1 = positions.get(entity1).unwrap();
        assert_eq!(pos1.x, 15);
        assert_eq!(pos1.y, 30);
    }

    #[test_case]
    fn test_component_container_basic_operations(_agb: &mut agb::Gba) {
        let mut container = VecComponentContainer::<TestPosition>::new();
        let entity1 = Entity::new(0);
        let entity2 = Entity::new(1);

        container.add_entity(entity1);
        container.add_entity(entity2);

        assert_eq!(container.len(), 2);

        let pos1 = TestPosition { x: 10, y: 20 };
        let pos2 = TestPosition { x: 30, y: 40 };

        container.set(entity1, pos1);
        container.set(entity2, pos2);

        let retrieved1 = container.get(entity1).unwrap();
        assert_eq!(retrieved1.x, 10);
        assert_eq!(retrieved1.y, 20);

        let retrieved2 = container.get(entity2).unwrap();
        assert_eq!(retrieved2.x, 30);
        assert_eq!(retrieved2.y, 40);
    }

    #[test_case]
    fn test_sparse_traversal_order(_agb: &mut agb::Gba) {
        let mut container = VecComponentContainer::<TestPosition>::new();

        let entity0 = Entity::new(0);
        let entity1 = Entity::new(1);
        let entity2 = Entity::new(2);

        container.add_entity(entity0);
        container.add_entity(entity1);
        container.add_entity(entity2);

        container.set(entity0, TestPosition { x: 0, y: 0 });
        container.set(entity1, TestPosition { x: 1, y: 1 });
        container.set(entity2, TestPosition { x: 2, y: 2 });

        let mut visited = Vec::new();
        container.for_each_sparse(|index, pos| {
            visited.push((index, pos.x, pos.y));
        });

        assert_eq!(visited.len(), 3);
        assert_eq!(visited[0], (0, 0, 0));
        assert_eq!(visited[1], (1, 1, 1));
        assert_eq!(visited[2], (2, 2, 2));
    }

    #[test_case]
    fn test_sparse_traversal_mutable(_agb: &mut agb::Gba) {
        let mut container = VecComponentContainer::<TestPosition>::new();

        let entity0 = Entity::new(0);
        let entity1 = Entity::new(1);

        container.add_entity(entity0);
        container.add_entity(entity1);

        container.set(entity0, TestPosition { x: 0, y: 0 });
        container.set(entity1, TestPosition { x: 1, y: 1 });

        container.for_each_sparse_mut(|_index, pos| {
            pos.x += 10;
            pos.y += 20;
        });

        let pos0 = container.get(entity0).unwrap();
        assert_eq!(pos0.x, 10);
        assert_eq!(pos0.y, 20);

        let pos1 = container.get(entity1).unwrap();
        assert_eq!(pos1.x, 11);
        assert_eq!(pos1.y, 21);
    }

    #[test_case]
    fn test_dense_vs_sparse_traversal(_agb: &mut agb::Gba) {
        let mut container = VecComponentContainer::<TestPosition>::new();

        let entity0 = Entity::new(0);
        let entity2 = Entity::new(2);
        let entity4 = Entity::new(4);

        container.add_entity(entity0);
        container.add_entity(Entity::new(1));
        container.add_entity(entity2);
        container.add_entity(Entity::new(3));
        container.add_entity(entity4);

        container.set(entity0, TestPosition { x: 0, y: 0 });
        container.set(entity2, TestPosition { x: 2, y: 2 });
        container.set(entity4, TestPosition { x: 4, y: 4 });

        let mut dense_count = 0;
        container.for_each(|_index, _pos| {
            dense_count += 1;
        });

        let mut sparse_count = 0;
        container.for_each_sparse(|_index, _pos| {
            sparse_count += 1;
        });

        assert_eq!(dense_count, 3);
        assert_eq!(sparse_count, 3);

        let mut sparse_indices = Vec::new();
        container.for_each_sparse(|index, _pos| {
            sparse_indices.push(index);
        });

        assert_eq!(sparse_indices, vec![0, 2, 4]);
    }
}
