use gba_ecs_rs::{define_world, Component, With, Without, GetStorage, ComponentStorage};

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

fn main() {
    let mut world = World::new();

    // Create entities with different component combinations
    let player = world.spawn_entity();
    world.add_component(player, Position { x: 10.0, y: 20.0 });
    world.add_component(player, Velocity { dx: 1.0, dy: 0.0 });
    world.add_component(player, Health { value: 100 });

    let enemy = world.spawn_entity();
    world.add_component(enemy, Position { x: 50.0, y: 30.0 });
    world.add_component(enemy, Velocity { dx: -0.5, dy: 0.0 });
    world.add_component(enemy, Damage { value: 25 });

    let static_object = world.spawn_entity();
    world.add_component(static_object, Position { x: 100.0, y: 50.0 });

    // Example 1: Query positions with velocity (moving entities)
    println!("Moving entities:");
    for pos in world.query::<&Position, With<Velocity>>() {
        println!("  Position: ({}, {})", pos.x, pos.y);
    }

    // Example 2: Query positions without velocity (static entities)
    println!("\nStatic entities:");
    for pos in world.query::<&Position, Without<Velocity>>() {
        println!("  Position: ({}, {})", pos.x, pos.y);
    }

    // Example 3: Query entities with health but without damage (friendly entities)
    println!("\nFriendly entities:");
    for pos in world.query::<&Position, (With<Health>, Without<Damage>)>() {
        println!("  Position: ({}, {})", pos.x, pos.y);
    }

    // Example 4: Query entities with damage but without health (hostile entities)
    println!("\nHostile entities:");
    for pos in world.query::<&Position, (With<Damage>, Without<Health>)>() {
        println!("  Position: ({}, {})", pos.x, pos.y);
    }

    // Example 5: Update moving entities using filters
    println!("\nUpdating moving entities...");
    for (pos, vel) in world.query::<(&mut Position, &Velocity), ()>() {
        pos.x += vel.dx;
        pos.y += vel.dy;
        println!("  Updated position: ({}, {})", pos.x, pos.y);
    }
}