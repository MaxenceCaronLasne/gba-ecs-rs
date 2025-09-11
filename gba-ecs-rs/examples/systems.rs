use gba_ecs_rs::{define_world, Component, With, Without, Query, GetStorage, ComponentStorage};

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

    // Create entities
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

    println!("=== Initial State ===");
    println!("Player position: {:?}", world.get_component::<Position>(player).unwrap());
    println!("Enemy position: {:?}", world.get_component::<Position>(enemy).unwrap());
    println!("NPC position: {:?}", world.get_component::<Position>(npc).unwrap());
    
    println!("Player health: {}", world.get_component::<Health>(player).unwrap().value);
    println!("Enemy health: {}", world.get_component::<Health>(enemy).unwrap().value);
    println!("NPC health: {}", world.get_component::<Health>(npc).unwrap().value);

    println!("\n=== Running Movement System ===");
    movement_system(world.query());

    println!("Player position after movement: {:?}", world.get_component::<Position>(player).unwrap());
    println!("Enemy position after movement: {:?}", world.get_component::<Position>(enemy).unwrap());
    println!("NPC position after movement: {:?}", world.get_component::<Position>(npc).unwrap());

    println!("\n=== Running Damage System (affects entities with Damage) ===");
    damage_system(world.query());
    
    println!("\n=== Running Healing System (affects entities without Damage) ===");
    healing_system(world.query());

    println!("\n=== Running V2 Systems ===");
    damage_system_v2(world.query());
    healing_system_v2(world.query());

    println!("\n=== Final State ===");
    println!("Player health: {}", world.get_component::<Health>(player).unwrap().value);
    println!("Enemy health: {}", world.get_component::<Health>(enemy).unwrap().value);
    println!("NPC health: {}", world.get_component::<Health>(npc).unwrap().value);
}