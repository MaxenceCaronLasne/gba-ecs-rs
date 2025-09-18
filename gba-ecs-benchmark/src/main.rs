#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;
use agb::Gba;
use gba_ecs_rs::{define_world, Component, ComponentStorage, Entity, GetStorage, Query};

mod bench;

#[derive(Component, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Clone, Copy)]
struct Velocity {
    dx: i32,
    dy: i32,
}

#[derive(Component, Clone, Copy)]
struct Health {
    value: i32,
}

define_world!(World {
    Position,
    Velocity,
    Health,
});

const ENTITY_COUNT: usize = 500;
const ITERATIONS: usize = 1;

fn setup_world(world: &mut World) {
    agb::println!("Setting up {} entities...", ENTITY_COUNT);

    for i in 0..ENTITY_COUNT {
        let entity = world.spawn_entity();

        world.add_component(
            entity,
            Position {
                x: i as i32,
                y: (i as i32) * 2,
            },
        );

        // Add velocity to every other entity
        if i % 2 == 0 {
            world.add_component(
                entity,
                Velocity {
                    dx: (i as i32) / 10,
                    dy: (i as i32) / 5,
                },
            );
        }

        // Add health to every third entity
        if i % 3 == 0 {
            world.add_component(
                entity,
                Health {
                    value: 100 + (i as i32),
                },
            );
        }
    }

    agb::println!("Setup complete!");
}

fn sum_position(query: Query<&Position, (), World>) -> i32 {
    let mut sum: i32 = 0;
    for q in query {
        sum += q.x + q.y;
    }

    sum
}

fn sum_velocity(query: Query<&Velocity, (), World>) -> i32 {
    let mut sum: i32 = 0;
    for q in query {
        sum += q.dx + q.dy;
    }

    sum
}

fn sum_position_raw(table: [Position; ENTITY_COUNT]) -> i32 {
    let mut sum: i32 = 0;
    for p in table {
        sum += p.x + p.y;
    }

    sum
}

#[agb::entry]
fn main(mut gba: Gba) -> ! {
    let mut world = World::new();

    // Initialize timing - use the correct AGB timer API
    let mut timers = gba.timers.timers();
    bench::init(&mut timers);

    let positions = [Position { x: 0, y: 0 }; ENTITY_COUNT];

    agb::println!("=== ECS Benchmark: Raw vs Query Iteration ===");
    agb::println!("Entity count: {}", ENTITY_COUNT);
    agb::println!("Iterations per test: {}", ITERATIONS);

    bench::start("setup");
    setup_world(&mut world);
    bench::stop("setup");

    bench::start("sumpos");
    let res = sum_position(world.query());
    bench::stop("sumpos");

    bench::start("sumposraw");
    let res = sum_position_raw(positions);
    bench::stop("sumposraw");

    bench::start("sumvel");
    let res = sum_velocity(world.query());
    bench::stop("sumvel");

    agb::println!("\n=== Results === {}", res);
    bench::log();

    agb::println!("\nBenchmark complete! Check results above.");
    agb::println!("Lower average times indicate better performance.");

    loop {}
}

