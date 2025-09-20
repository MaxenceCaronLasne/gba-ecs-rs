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
    fn add_component<C>(&mut self, entity: Entity, component: C)
    where
        Self: GetComponentContainer<C>;
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

struct ZipWhere<A, B, IterA, IterB>
where
    IterA: Iterator<Item = (usize, A)>,
    IterB: Iterator<Item = (usize, B)>,
{
    iter_a: IterA,
    iter_b: IterB,
    current_a: Option<(usize, A)>,
    current_b: Option<(usize, B)>,
}

impl<A, B, IterA, IterB> ZipWhere<A, B, IterA, IterB>
where
    IterA: Iterator<Item = (usize, A)>,
    IterB: Iterator<Item = (usize, B)>,
{
    fn new(mut iter_a: IterA, mut iter_b: IterB) -> Self {
        let current_a = iter_a.next();
        let current_b = iter_b.next();

        Self {
            iter_a,
            iter_b,
            current_a,
            current_b,
        }
    }
}

impl<A, B, IterA, IterB> Iterator for ZipWhere<A, B, IterA, IterB>
where
    IterA: Iterator<Item = (usize, A)>,
    IterB: Iterator<Item = (usize, B)>,
{
    type Item = (usize, (A, B));

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (&self.current_a, &self.current_b) {
                (Some((key_a, _)), Some((key_b, _))) => match key_a.cmp(key_b) {
                    core::cmp::Ordering::Equal => {
                        let a = self.current_a.take().unwrap();
                        let b = self.current_b.take().unwrap();
                        self.current_a = self.iter_a.next();
                        self.current_b = self.iter_b.next();
                        return Some((a.0, (a.1, b.1)));
                    }
                    core::cmp::Ordering::Less => {
                        self.current_a = self.iter_a.next();
                    }
                    core::cmp::Ordering::Greater => {
                        self.current_b = self.iter_b.next();
                    }
                },
                _ => return None,
            }
        }
    }
}

fn zip_where<A, B, IterA, IterB>(iter_a: IterA, iter_b: IterB) -> ZipWhere<A, B, IterA, IterB>
where
    IterA: Iterator<Item = (usize, A)>,
    IterB: Iterator<Item = (usize, B)>,
{
    ZipWhere::new(iter_a, iter_b)
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

    bench::start("ecs");
    for (i, p) in positions.iter() {
        sum += p.x + p.y;
    }
    bench::stop("ecs");

    bench::start("table");
    for (i, p) in table.iter().enumerate().filter_map(|(i, p)| match p {
        Some(value) => Some((i, value)),
        None => None,
    }) {
        sum += p.x + p.y;
    }
    bench::stop("table");

    for (i, p) in positions.iter() {
        agb::println!("{}: p={:?}", i, p);
    }

    for (i, v) in velocities.iter() {
        agb::println!("{}: v={:?}", i, v);
    }

    agb::println!(
        "is sorted {}",
        strongness.iter().is_sorted_by(|(ia, _), (ib, _)| ia < ib)
    );

    for (i, s) in strongness.iter() {
        agb::println!("{}: s={:?}", i, s);
    }

    for (i, (p, v)) in zip_where(positions.iter(), velocities.iter()) {
        agb::println!("{}: p={:?}, v={:?}", i, p, v);
    }

    agb::println!("sum={}", sum);
    bench::log();
    loop {}
}
