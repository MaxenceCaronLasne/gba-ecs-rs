use gba_ecs_rs::{
    define_world, Component, ComponentStorage, Entity, GetStorage, Query, With, Without,
};
use std::time::Instant;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Component, Debug)]
struct Health {
    value: i32,
}

#[derive(Component, Debug)]
struct Damage {
    value: i32,
}

define_world!(World {
    Position,
    Velocity,
    Health,
    Damage,
});

// System functions that work cleanly
fn movement_system(query: Query<(&mut Position, &Velocity), (), World>) {
    for (pos, vel) in query {
        pos.x += vel.dx;
        pos.y += vel.dy;
    }
}

fn damage_system(query: Query<&mut Health, With<Damage>, World>) {
    for health in query {
        health.value -= 5;
        println!("Dealing 5 damage, health now: {}", health.value);
    }
}

fn healing_system(query: Query<&mut Health, Without<Damage>, World>) {
    for health in query {
        health.value += 2;
        println!("Healing for 2, health now: {}", health.value);
    }
}

// Alternative system signatures
fn damage_system_v2(query: Query<&mut Health, With<Damage>, World>) {
    for health in query {
        health.value -= 3;
        println!("V2: Dealing 3 damage, health now: {}", health.value);
    }
}

fn healing_system_v2(query: Query<&mut Health, Without<Damage>, World>) {
    for health in query {
        health.value += 1;
        println!("V2: Healing for 1, health now: {}", health.value);
    }
}

fn main() {
    let mut world = World::new();

    let player = world.spawn_entity();
    world.add_component(player, Position { x: 10.0, y: 20.0 });
    world.add_component(player, Velocity { dx: 1.0, dy: 0.5 });
    world.add_component(player, Health { value: 100 });

    let enemy = world.spawn_entity();
    world.add_component(enemy, Position { x: 50.0, y: 30.0 });
    world.add_component(enemy, Velocity { dx: -0.5, dy: 0.0 });
    world.add_component(enemy, Health { value: 75 });
    world.add_component(enemy, Damage { value: 25 });

    let npc = world.spawn_entity();
    world.add_component(npc, Position { x: 100.0, y: 50.0 });
    world.add_component(npc, Health { value: 50 });

    damage_system_v2(world.query());

    let start = Instant::now();
    for _ in 0..1000 {
        for pos in world.query::<&mut Position, ()>() {
            pos.x += pos.x;
        }
    }
    println!("Query time: {:?}", start.elapsed());

    // Time manual approach
    let start = Instant::now();
    for _ in 0..1000 {
        for i in 0..world.entity_count {
            let entity = Entity { index: i };
            if let Some(pos) = world.get_component_mut::<Position>(entity) {
                pos.x += pos.x;
            }
        }
    }
    println!("Manual time: {:?}", start.elapsed());
}
