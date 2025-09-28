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

use gba_ecs_rs::{world, zip3, ComponentContainer, Entity, World};

mod bench;

#[derive(Clone, Copy, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Default)]
struct Velocity {
    dx: i32,
    dy: i32,
}

#[derive(Clone, Copy, Debug, Default)]
struct Strongness {
    value: i32,
}

world!(MyWorld {
    Position,
    Velocity,
    Strongness
});

const ITERATIONS: usize = 1000;

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
            },
        );
        if i.is_multiple_of(2) {
            world.add_component(
                entity,
                Velocity {
                    dx: 0,
                    dy: (i as i32),
                },
            );
        }
        if i.is_multiple_of(8) {
            world.add_component(entity, Strongness { value: i as i32 });
        }
        table[i] = Some(Position {
            x: (i as i32),
            y: 0,
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
    positions.for_each(|_, p| sum += p.x + p.y);
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
    use gba_ecs_rs::VecComponentContainer;

    // Test components - independent from main program components
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

    // Test the world! macro
    world!(MacroTestWorld {
        TestPosition,
        TestVelocity
    });

    #[test_case]
    fn test_world_macro(_agb: &mut agb::Gba) {
        let mut world = MacroTestWorld::new();

        // Test add_entity
        let entity1 = world.add_entity();
        let entity2 = world.add_entity();

        // Test add_component
        world.add_component(entity1, TestPosition { x: 10, y: 20 });
        world.add_component(entity1, TestVelocity { dx: 1, dy: 2 });
        world.add_component(entity2, TestPosition { x: 30, y: 40 });

        // Test get_components
        let positions = world.get_components::<TestPosition>();
        let velocities = world.get_components::<TestVelocity>();

        // Verify positions
        let pos1 = positions.get(entity1).unwrap();
        assert_eq!(pos1.x, 10);
        assert_eq!(pos1.y, 20);

        let pos2 = positions.get(entity2).unwrap();
        assert_eq!(pos2.x, 30);
        assert_eq!(pos2.y, 40);

        // Verify velocities
        let vel1 = velocities.get(entity1).unwrap();
        assert_eq!(vel1.dx, 1);
        assert_eq!(vel1.dy, 2);

        // Entity2 should not have velocity
        assert!(velocities.get(entity2).is_none());

        // Test get_components_mut
        let positions_mut = world.get_components_mut::<TestPosition>();
        if let Some(pos) = positions_mut.get_mut(entity1) {
            pos.x += 5;
            pos.y += 10;
        }

        // Verify mutation
        let positions = world.get_components::<TestPosition>();
        let pos1 = positions.get(entity1).unwrap();
        assert_eq!(pos1.x, 15);
        assert_eq!(pos1.y, 30);
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

        // Add entities in order: 0, 1, 2
        let entity0 = Entity::new(0);
        let entity1 = Entity::new(1);
        let entity2 = Entity::new(2);

        container.add_entity(entity0);
        container.add_entity(entity1);
        container.add_entity(entity2);

        // Set components in order: 0, 1, 2
        container.set(entity0, TestPosition { x: 0, y: 0 });
        container.set(entity1, TestPosition { x: 1, y: 1 });
        container.set(entity2, TestPosition { x: 2, y: 2 });

        // Sparse traversal should visit in reverse order (2, 1, 0) due to head insertion
        let mut visited = Vec::new();
        container.for_each_sparse(|index, pos| {
            visited.push((index, pos.x, pos.y));
        });

        assert_eq!(visited.len(), 3);
        assert_eq!(visited[0], (0, 0, 0)); // First inserted (tail)
        assert_eq!(visited[1], (1, 1, 1)); // Middle
        assert_eq!(visited[2], (2, 2, 2)); // Last inserted (head)
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
        container.set(entity0, TestPosition { x: 0, y: 0 });
        container.set(entity2, TestPosition { x: 2, y: 2 });
        container.set(entity4, TestPosition { x: 4, y: 4 });

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

        assert_eq!(sparse_indices, vec![0, 2, 4]);
    }
}
