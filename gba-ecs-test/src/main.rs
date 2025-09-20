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

use agb::println;
use agb::Gba;
use alloc::vec::Vec;
use gba_ecs_rs::{
    ComponentContainer, DenseComponentContainer, DenseMarkerContainer, Entity,
    GetComponentContainer, MarkerContainer, SparseComponentContainer, SparseMarkerContainer,
};

mod bench;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Velocity {
    dx: i32,
    dy: i32,
}

#[derive(Clone, Copy, Debug)]
struct Strongness(i32);

trait World {
    fn new() -> Self;
    fn add_entity(&mut self) -> Entity;
}

struct MyWorld {
    last_entity: usize,
    is_enemy: SparseMarkerContainer,
    positions: DenseComponentContainer<Position>,
    velocities: DenseComponentContainer<Velocity>,
    strongness: SparseComponentContainer<Strongness>,
}

impl World for MyWorld {
    fn new() -> Self {
        Self {
            last_entity: 0,
            is_enemy: SparseMarkerContainer::new(),
            positions: DenseComponentContainer::new(),
            velocities: DenseComponentContainer::new(),
            strongness: SparseComponentContainer::new(),
        }
    }

    fn add_entity(&mut self) -> Entity {
        let entity = Entity::new(self.last_entity);
        self.last_entity += 1;

        self.is_enemy.add_entity(entity);
        self.positions.add_entity(entity);
        self.velocities.add_entity(entity);
        self.strongness.add_entity(entity);

        entity
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

    fn add_component<C>(&mut self, entity: Entity, component: C)
    where
        Self: GetComponentContainer<C>,
    {
        let mut container = self.get_components_mut::<C>();
        container.set(entity, component);
    }
}

impl GetComponentContainer<Position> for MyWorld {
    type Container = DenseComponentContainer<Position>;
    fn get_components(&self) -> &Self::Container {
        &self.positions
    }
    fn get_components_mut(&mut self) -> &mut Self::Container {
        &mut self.positions
    }
}

impl GetComponentContainer<Velocity> for MyWorld {
    type Container = DenseComponentContainer<Velocity>;
    fn get_components(&self) -> &Self::Container {
        &self.velocities
    }
    fn get_components_mut(&mut self) -> &mut Self::Container {
        &mut self.velocities
    }
}

impl GetComponentContainer<Strongness> for MyWorld {
    type Container = SparseComponentContainer<Strongness>;
    fn get_components(&self) -> &Self::Container {
        &self.strongness
    }
    fn get_components_mut(&mut self) -> &mut Self::Container {
        &mut self.strongness
    }
}

const ITERATIONS: usize = 100;

// The main function must take 1 arguments and never returns, and must be marked with
// the #[agb::entry] macro.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut timers = gba.timers.timers();
    bench::init(&mut timers);
    let mut world = MyWorld::new();

    let mut raw_table: [Option<Position>; ITERATIONS] = [None; ITERATIONS];
    let mut table = &mut raw_table;

    for i in 0..ITERATIONS {
        let entity = world.add_entity();
        world.add_component(
            entity,
            Position {
                x: (i as i32),
                y: 0,
            },
        );
        table[i] = Some(Position {
            x: (i as i32),
            y: 0,
        });
    }

    let positions = world.get_components::<Position>();

    let mut sum = 0;

    bench::start("ecs");
    for (i, p) in positions.iter() {
        sum += p.x + p.y;
    }
    bench::stop("ecs");

    bench::start("table");
    for (i, p) in table.iter().flatten().enumerate() {
        sum += p.x + p.y;
    }
    bench::stop("table");

    agb::println!("sum={}", sum);
    bench::log();
    loop {}
}
