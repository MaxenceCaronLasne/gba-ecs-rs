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

use core::primitive;

use agb::println;
use agb::Gba;
use alloc::vec::Vec;
use gba_ecs_rs::{zip, zip3, ComponentContainer, VecComponentContainer, Entity, GetComponentContainer};

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
        let mut container = self.get_components_mut::<C>();
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
        if i % 2 == 0 {
            world.add_component(
                entity,
                Velocity {
                    dx: 0,
                    dy: (i as i32),
                },
            );
        }
        if i % 8 == 0 {
            world.add_component(entity, Strongness(i as i32));
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
    positions.for_each(|i, p| sum += p.x + p.y);
    bench::stop("ecs");

    bench::start("double");
    zip3(positions, velocities, strongness).for_each_mut(|i, p, v, s| sum += p.x + v.dx + s.0);
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
