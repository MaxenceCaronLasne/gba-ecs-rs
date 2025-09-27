// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't have an operating
// system, so most of the content of the standard library doesn't apply.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

// By default no_std crates don't get alloc, so you won't be able to use things like Vec
// until you declare the extern crate. `agb` provides an allocator so it will all work
extern crate alloc;

use gba_ecs_rs::{
    zip3, Component, ComponentContainer, Entity, GetComponentContainer, VecComponentContainer,
};

mod bench;

#[derive(Component, Clone, Copy, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Component, Clone, Copy, Debug, Default)]
struct Velocity {
    dx: i32,
    dy: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Component, Clone, Copy, Debug, Default)]
struct Strongness {
    value: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

trait World {
    fn new() -> Self;
    fn add_entity(&mut self) -> Entity;
    fn add_component<C>(&mut self, entity: Entity, component: C)
    where
        Self: GetComponentContainer<C>;
}

struct MyWorld {
    last_entity: usize,
    positions: VecComponentContainer<Position>,
    velocities: VecComponentContainer<Velocity>,
    strongness: VecComponentContainer<Strongness>,
}

impl World for MyWorld {
    fn new() -> Self {
        Self {
            last_entity: 0,
            positions: VecComponentContainer::new(),
            velocities: VecComponentContainer::new(),
            strongness: VecComponentContainer::new(),
        }
    }

    fn add_entity(&mut self) -> Entity {
        let entity = Entity::new(self.last_entity);
        self.last_entity += 1;

        self.positions.add_entity(entity);
        self.velocities.add_entity(entity);
        self.strongness.add_entity(entity);

        entity
    }

    fn add_component<C>(&mut self, entity: Entity, component: C)
    where
        Self: GetComponentContainer<C>,
    {
        let container = self.get_components_mut::<C>();
        container.set(entity, component);
    }
}

impl MyWorld {
    fn get_components<C>(&self) -> &<Self as GetComponentContainer<C>>::Container
    where
        Self: GetComponentContainer<C>,
    {
        GetComponentContainer::get_components(self)
    }

    fn get_components_mut<C>(&mut self) -> &mut <Self as GetComponentContainer<C>>::Container
    where
        Self: GetComponentContainer<C>,
    {
        GetComponentContainer::get_components_mut(self)
    }
}

impl GetComponentContainer<Position> for MyWorld {
    type Container = VecComponentContainer<Position>;
    fn get_components(&self) -> &Self::Container {
        &self.positions
    }
    fn get_components_mut(&mut self) -> &mut Self::Container {
        &mut self.positions
    }
}

impl GetComponentContainer<Velocity> for MyWorld {
    type Container = VecComponentContainer<Velocity>;
    fn get_components(&self) -> &Self::Container {
        &self.velocities
    }
    fn get_components_mut(&mut self) -> &mut Self::Container {
        &mut self.velocities
    }
}

impl GetComponentContainer<Strongness> for MyWorld {
    type Container = VecComponentContainer<Strongness>;
    fn get_components(&self) -> &Self::Container {
        &self.strongness
    }
    fn get_components_mut(&mut self) -> &mut Self::Container {
        &mut self.strongness
    }
}

const ITERATIONS: usize = 1000;

// The main function must take 1 arguments and never returns, and must be marked with
// the #[agb::entry] macro.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut timers = gba.timers.timers();
    bench::init(&mut timers);
    let mut world = MyWorld::new();

    let mut raw_table: [Option<Position>; ITERATIONS] = [None; ITERATIONS];
    let table = &mut raw_table;

    for i in 0..ITERATIONS {
        let entity = world.add_entity();
        world.add_component(
            entity,
            Position {
                x: (i as i32),
                y: 0,
                ..Default::default()
            },
        );
        if i.is_multiple_of(2) {
            world.add_component(
                entity,
                Velocity {
                    dx: 0,
                    dy: (i as i32),
                    next: None,
                    prev: None,
                },
            );
        }
        if i.is_multiple_of(8) {
            world.add_component(
                entity,
                Strongness {
                    value: i as i32,
                    next: None,
                    prev: None,
                },
            );
        }
        table[i] = Some(Position {
            x: (i as i32),
            y: 0,
            ..Default::default()
        });
    }

    let positions = world.get_components::<Position>();
    let velocities = world.get_components::<Velocity>();
    let strongness = world.get_components::<Strongness>();

    let mut sum = 0;

    bench::start("ecs base");
    for i in 0..ITERATIONS {
        if let Some(p) = positions.get(Entity::new(i)) {
            sum += p.x + p.y;
        }
    }
    bench::stop("ecs base");

    bench::start("ecs");
    positions.for_each_sparse(|_, p| sum += p.x + p.y);
    bench::stop("ecs");

    bench::start("double");
    zip3(positions, velocities, strongness).for_each_mut(|_, p, v, s| sum += p.x + v.dx + s.value);
    bench::stop("double");

    bench::start("double base");
    for i in 0..ITERATIONS {
        if let Some(p) = positions.get(Entity::new(i)) {
            if let Some(v) = velocities.get(Entity::new(i)) {
                sum += p.x + v.dx;
            }
        }
    }
    bench::stop("double base");

    agb::println!("sum={}", sum);
    bench::log();

    loop {}
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    // Test components - independent from main program components
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestPosition {
        x: i32,
        y: i32,
        next: Option<usize>,
        prev: Option<usize>,
    }

    impl Component for TestPosition {
        fn next(&self) -> Option<usize> {
            self.next
        }
        fn prev(&self) -> Option<usize> {
            self.prev
        }
        fn set_next(&mut self, next: Option<usize>) {
            self.next = next;
        }
        fn set_prev(&mut self, prev: Option<usize>) {
            self.prev = prev;
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestVelocity {
        dx: i32,
        dy: i32,
        next: Option<usize>,
        prev: Option<usize>,
    }

    impl Component for TestVelocity {
        fn next(&self) -> Option<usize> {
            self.next
        }
        fn prev(&self) -> Option<usize> {
            self.prev
        }
        fn set_next(&mut self, next: Option<usize>) {
            self.next = next;
        }
        fn set_prev(&mut self, prev: Option<usize>) {
            self.prev = prev;
        }
    }

    #[test_case]
    fn test_component_container_basic_operations(_agb: &mut agb::Gba) {
        let mut container = VecComponentContainer::<TestPosition>::new();
        let entity1 = Entity::new(0);
        let entity2 = Entity::new(1);

        // Test add_entity
        container.add_entity(entity1);
        container.add_entity(entity2);

        assert_eq!(container.len(), 2);

        // Test set and get
        let pos1 = TestPosition {
            x: 10,
            y: 20,
            next: None,
            prev: None,
        };
        let pos2 = TestPosition {
            x: 30,
            y: 40,
            next: None,
            prev: None,
        };

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

        // Add entities in order: 0, 1, 2
        let entity0 = Entity::new(0);
        let entity1 = Entity::new(1);
        let entity2 = Entity::new(2);

        container.add_entity(entity0);
        container.add_entity(entity1);
        container.add_entity(entity2);

        // Set components in order: 0, 1, 2
        container.set(
            entity0,
            TestPosition {
                x: 0,
                y: 0,
                next: None,
                prev: None,
            },
        );
        container.set(
            entity1,
            TestPosition {
                x: 1,
                y: 1,
                next: None,
                prev: None,
            },
        );
        container.set(
            entity2,
            TestPosition {
                x: 2,
                y: 2,
                next: None,
                prev: None,
            },
        );

        // Sparse traversal should visit in reverse order (2, 1, 0) due to head insertion
        let mut visited = Vec::new();
        container.for_each_sparse(|index, pos| {
            visited.push((index, pos.x, pos.y));
        });

        assert_eq!(visited.len(), 3);
        assert_eq!(visited[0], (2, 2, 2)); // Last inserted (head)
        assert_eq!(visited[1], (1, 1, 1)); // Middle
        assert_eq!(visited[2], (0, 0, 0)); // First inserted (tail)
    }

    #[test_case]
    fn test_sparse_traversal_mutable(_agb: &mut agb::Gba) {
        let mut container = VecComponentContainer::<TestPosition>::new();

        let entity0 = Entity::new(0);
        let entity1 = Entity::new(1);

        container.add_entity(entity0);
        container.add_entity(entity1);

        container.set(
            entity0,
            TestPosition {
                x: 0,
                y: 0,
                next: None,
                prev: None,
            },
        );
        container.set(
            entity1,
            TestPosition {
                x: 1,
                y: 1,
                next: None,
                prev: None,
            },
        );

        // Modify through sparse traversal
        container.for_each_sparse_mut(|_index, pos| {
            pos.x += 10;
            pos.y += 20;
        });

        // Verify modifications
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

        // Create sparse container with gaps
        let entity0 = Entity::new(0);
        let entity2 = Entity::new(2);
        let entity4 = Entity::new(4);

        container.add_entity(entity0);
        container.add_entity(Entity::new(1)); // Add but don't set
        container.add_entity(entity2);
        container.add_entity(Entity::new(3)); // Add but don't set
        container.add_entity(entity4);

        // Only set components for 0, 2, 4 (creating gaps at 1, 3)
        container.set(
            entity0,
            TestPosition {
                x: 0,
                y: 0,
                next: None,
                prev: None,
            },
        );
        container.set(
            entity2,
            TestPosition {
                x: 2,
                y: 2,
                next: None,
                prev: None,
            },
        );
        container.set(
            entity4,
            TestPosition {
                x: 4,
                y: 4,
                next: None,
                prev: None,
            },
        );

        // Dense traversal should visit all 3 components
        let mut dense_count = 0;
        container.for_each(|_index, _pos| {
            dense_count += 1;
        });

        // Sparse traversal should also visit all 3 components
        let mut sparse_count = 0;
        container.for_each_sparse(|_index, _pos| {
            sparse_count += 1;
        });

        assert_eq!(dense_count, 3);
        assert_eq!(sparse_count, 3);

        // But sparse should visit in reverse insertion order: 4, 2, 0
        let mut sparse_indices = Vec::new();
        container.for_each_sparse(|index, _pos| {
            sparse_indices.push(index);
        });

        assert_eq!(sparse_indices, vec![4, 2, 0]);
    }

    #[test_case]
    fn test_linked_list_integrity(_agb: &mut agb::Gba) {
        let mut container = VecComponentContainer::<TestPosition>::new();

        // Add several entities
        for i in 0..5 {
            let entity = Entity::new(i);
            container.add_entity(entity);
            container.set(
                entity,
                TestPosition {
                    x: i as i32,
                    y: (i * 2) as i32,
                    next: None,
                    prev: None,
                },
            );
        }

        // Check that linked list connections are correct
        let mut current_index = 4; // Should start from last inserted (head)
        let mut count = 0;

        container.for_each_sparse(|index, pos| {
            assert_eq!(index, current_index);
            assert_eq!(pos.x, current_index as i32);
            assert_eq!(pos.y, (current_index * 2) as i32);

            if current_index > 0 {
                // Check that this component points to the previous one
                assert_eq!(pos.next(), Some(current_index - 1));
            } else {
                // Last component should have no next
                assert_eq!(pos.next(), None);
            }

            if count > 0 {
                // Non-head components should have a prev pointer
                assert_eq!(pos.prev(), Some(current_index + 1));
            } else {
                // Head component should have no prev
                assert_eq!(pos.prev(), None);
            }

            current_index = current_index.wrapping_sub(1);
            count += 1;
        });

        assert_eq!(count, 5);
    }
}
