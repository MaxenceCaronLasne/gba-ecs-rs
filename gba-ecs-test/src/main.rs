#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;
use agb::ExternalAllocator;
use alloc::boxed::Box;
use alloc::vec::Vec;
use gba_ecs_rs::{world, zip, zip3, ComponentContainer, Entity, World};

mod bench;
mod tests;

#[derive(Clone, Copy, Debug, Default)]
struct Modulo1(i32);

#[derive(Clone, Copy, Debug, Default)]
struct Modulo2(i32);

#[derive(Clone, Copy, Debug, Default)]
struct Modulo8(i32);

#[derive(Clone, Copy, Debug, Default)]
struct Unique(i32);

world!(MyWorld {
    (Modulo1, ExternalAllocator),
    (Modulo2, ExternalAllocator),
    (Modulo8, ExternalAllocator),
    (Unique, ExternalAllocator),
});

const ITERATIONS: usize = 1000;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut timers = gba.timers.timers();
    bench::init(&mut timers);
    let mut world = MyWorld::new();

    let mut modulo1_vec = Vec::<Option<Modulo1>>::new();
    let mut modulo2_vec = Vec::<Option<Modulo2>>::new();
    let mut modulo8_vec = Vec::<Option<Modulo8>>::new();
    let unique = Box::new(Some(Unique(0)));

    for i in 0..ITERATIONS {
        let entity = world.add_entity();

        world.add_component(entity, Modulo1(i as i32));
        modulo1_vec.push(Some(Modulo1(i as i32)));

        if i == ITERATIONS / 2 {
            world.add_component(entity, Unique(i as i32));
        }

        if i.is_multiple_of(2) {
            world.add_component(entity, Modulo2(2 * i as i32));
            modulo2_vec.push(Some(Modulo2(2 * i as i32)));
        } else {
            modulo2_vec.push(None);
        }

        if i.is_multiple_of(8) {
            world.add_component(entity, Modulo8(8 * i as i32));
            modulo8_vec.push(Some(Modulo8(8 * i as i32)));
        } else {
            modulo8_vec.push(None);
        }
    }

    let modulo1_container = world.get_components::<Modulo1>();
    let modulo2_container = world.get_components::<Modulo2>();
    let modulo8_container = world.get_components::<Modulo8>();
    let unique_container = world.get_components::<Unique>();

    let mut sum = 0;

    bench::start("table modulo1");
    modulo1_vec
        .iter()
        .filter_map(|i| *i)
        .for_each(|i| sum += i.0);
    bench::stop("table modulo1");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("container modulo1");
    for i in 0..ITERATIONS {
        if let Some(m1) = modulo1_container.get(Entity::new(i)) {
            sum += m1.0;
        }
    }
    bench::stop("container modulo1");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("for_each modulo1");
    modulo1_container.for_each(|_index, m1| {
        sum += m1.0;
    });
    bench::stop("for_each modulo1");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("table mod1 + mod2");
    modulo1_vec
        .iter()
        .zip(modulo2_vec.iter())
        .filter_map(|(m1_opt, m2_opt)| {
            if let (Some(m1), Some(m2)) = (m1_opt, m2_opt) {
                Some((m1, m2))
            } else {
                None
            }
        })
        .for_each(|(m1, m2)| {
            sum += m1.0 + m2.0;
        });
    bench::stop("table mod1 + mod2");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("container mod1 + mod2");
    for i in 0..ITERATIONS {
        if let (Some(m1), Some(m2)) = (
            modulo1_container.get(Entity::new(i)),
            modulo2_container.get(Entity::new(i)),
        ) {
            sum += m1.0 + m2.0;
        }
    }
    bench::stop("container mod1 + mod2");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("for_each mod1 + mod2");
    zip(modulo1_container, modulo2_container).for_each(|_e, m1, m2| {
        sum += m1.0 + m2.0;
    });
    bench::stop("for_each mod1 + mod2");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("for_each_sparse mod1 + mod2");
    zip(modulo1_container, modulo2_container).for_each_sparse_mut(|_e, m1, m2| {
        sum += m1.0 + m2.0;
    });
    bench::stop("for_each_sparse mod1 + mod2");
    agb::println!("sum={}", sum);

    bench::start("table mod1 + mod2 + mod8");
    modulo1_vec
        .iter()
        .zip(modulo2_vec.iter())
        .zip(modulo8_vec.iter())
        .filter_map(|((m1_opt, m2_opt), m8_opt)| {
            if let (Some(m1), Some(m2), Some(m8)) = (m1_opt, m2_opt, m8_opt) {
                Some((m1, m2, m8))
            } else {
                None
            }
        })
        .for_each(|(m1, m2, m8)| {
            sum += m1.0 + m2.0 + m8.0;
        });
    bench::stop("table mod1 + mod2 + mod8");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("container mod1 + mod2 + mod8");
    for i in 0..ITERATIONS {
        if let (Some(m1), Some(m2), Some(m8)) = (
            modulo1_container.get(Entity::new(i)),
            modulo2_container.get(Entity::new(i)),
            modulo8_container.get(Entity::new(i)),
        ) {
            sum += m1.0 + m2.0 + m8.0;
        }
    }
    bench::stop("container mod1 + mod2 + mod8");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("for_each mod1 + mod2 + mod8");
    zip3(modulo1_container, modulo2_container, modulo8_container).for_each(|_e, m1, m2, m8| {
        sum += m1.0 + m2.0 + m8.0;
    });
    bench::stop("for_each mod1 + mod2 + mod8");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("for_each_sparse mod1 + mod2 + mod8");
    zip3(modulo1_container, modulo2_container, modulo8_container).for_each_sparse(
        |_e, m1, m2, m8| {
            sum += m1.0 + m2.0 + m8.0;
        },
    );
    bench::stop("for_each_sparse mod1 + mod2 + mod8");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("unique");
    zip(unique_container, modulo1_container).for_each(|_e, u, m1| {
        sum += u.0 + m1.0;
    });
    bench::stop("unique");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("unique sparse");
    zip(unique_container, modulo1_container).for_each_sparse_mut(|_e, u, m1| {
        sum += u.0 + m1.0;
    });
    bench::stop("unique sparse");
    agb::println!("sum={}", sum);
    sum = 0;

    bench::start("unique hand");
    let ou = *unique;
    let om1 = modulo1_container.get(Entity::new(ITERATIONS / 2));

    if let (Some(u), Some(m1)) = (ou, om1) {
        sum += u.0 + m1.0;
    }

    bench::stop("unique hand");
    agb::println!("sum={}", sum);

    bench::log();
    loop {
        agb::halt();
    }
}
