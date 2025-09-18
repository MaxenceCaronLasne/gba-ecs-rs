#![no_std]

mod component;
mod entity;
mod query;
mod storage;
mod world;

// Re-export procedural macros from the macro crate
pub use gba_ecs_macros::*;

// Re-export all public items from modules
pub use component::Component;
pub use entity::Entity;
pub use query::{ComponentQuery, DefaultFilter, FilterQuery, Query, QueryIterator, With, Without};
pub use storage::{ComponentStorage, GetStorage, VecStorage};
pub use world::WorldTrait;

#[cfg(test)]
mod tests {
    use super::*;
    extern crate alloc;
    use crate as gba_ecs_rs; // Alias for the macro to find the crate
    use alloc::vec::Vec;
    use gba_ecs_macros::{define_world, Component};

    #[derive(Component)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Component)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }

    #[derive(Component)]
    struct Health {
        value: i32,
    }

    define_world!(World {
        Position,
        Velocity,
        Health,
    });

    #[test]
    fn it_works() {
        let mut world = World::new();
        let entity = world.spawn_entity();
        world.add_component(entity, Position { x: 1.0, y: 0.0 });

        let pos = world.get_component::<Position>(entity).unwrap();
        assert_eq!(pos.x, 1.0);
    }

    #[test]
    fn test_multiple_components() {
        let mut world = World::new();
        let entity = world.spawn_entity();

        world.add_component(entity, Position { x: 1.0, y: 2.0 });
        world.add_component(entity, Velocity { dx: 0.5, dy: -0.5 });
        world.add_component(entity, Health { value: 100 });

        let pos = world.get_component::<Position>(entity).unwrap();
        let vel = world.get_component::<Velocity>(entity).unwrap();
        let health = world.get_component::<Health>(entity).unwrap();

        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(vel.dx, 0.5);
        assert_eq!(vel.dy, -0.5);
        assert_eq!(health.value, 100);

        {
            let pos_mut = world.get_component_mut::<Position>(entity).unwrap();
            pos_mut.x = 10.0;
        }

        let pos = world.get_component::<Position>(entity).unwrap();
        assert_eq!(pos.x, 10.0);

        let removed_health = world.remove_component::<Health>(entity);
        assert_eq!(removed_health.unwrap().value, 100);
        assert!(world.get_component::<Health>(entity).is_none());
    }

    #[test]
    fn test_single_component_query_readonly() {
        let mut world = World::new();

        // Create entities with Position components
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });

        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 3.0, y: 4.0 });

        // Entity without Position
        let _entity3 = world.spawn_entity();
        world.add_component(_entity3, Health { value: 50 });

        // Query all entities with Position
        let mut positions = Vec::new();
        for pos in world.query::<&Position, ()>() {
            positions.push((pos.x, pos.y));
        }

        // Should find exactly 2 positions
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&(1.0, 2.0)));
        assert!(positions.contains(&(3.0, 4.0)));
    }

    #[test]
    fn test_single_component_query_mutable() {
        let mut world = World::new();

        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });

        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 3.0, y: 4.0 });

        // Modify all positions through query
        for pos in world.query::<&mut Position, ()>() {
            pos.x += 10.0;
            pos.y += 20.0;
        }

        // Verify modifications
        let pos1 = world.get_component::<Position>(entity1).unwrap();
        assert_eq!(pos1.x, 11.0);
        assert_eq!(pos1.y, 22.0);

        let pos2 = world.get_component::<Position>(entity2).unwrap();
        assert_eq!(pos2.x, 13.0);
        assert_eq!(pos2.y, 24.0);
    }

    #[test]
    fn test_two_component_query() {
        let mut world = World::new();

        // Entity with Position and Velocity
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });
        world.add_component(entity1, Velocity { dx: 0.5, dy: -0.5 });

        // Entity with Position only
        let _entity2 = world.spawn_entity();
        world.add_component(_entity2, Position { x: 3.0, y: 4.0 });

        // Entity with Velocity only
        let _entity3 = world.spawn_entity();
        world.add_component(_entity3, Velocity { dx: 1.0, dy: 1.0 });

        // Query for entities with both Position and Velocity
        let mut results = Vec::new();
        for (pos, vel) in world.query::<(&Position, &Velocity), ()>() {
            results.push((pos.x, pos.y, vel.dx, vel.dy));
        }

        // Should only find entity1
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], (1.0, 2.0, 0.5, -0.5));
    }

    #[test]
    fn test_two_component_query_with_mutation() {
        let mut world = World::new();

        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 0.0, y: 0.0 });
        world.add_component(entity1, Velocity { dx: 1.0, dy: 2.0 });

        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 5.0, y: 5.0 });
        world.add_component(entity2, Velocity { dx: -1.0, dy: -2.0 });

        // Update positions based on velocity
        for (pos, vel) in world.query::<(&mut Position, &Velocity), ()>() {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }

        // Verify updates
        let pos1 = world.get_component::<Position>(entity1).unwrap();
        assert_eq!(pos1.x, 1.0);
        assert_eq!(pos1.y, 2.0);

        let pos2 = world.get_component::<Position>(entity2).unwrap();
        assert_eq!(pos2.x, 4.0);
        assert_eq!(pos2.y, 3.0);
    }

    #[test]
    fn test_three_component_query() {
        let mut world = World::new();

        // Entity with all three components
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });
        world.add_component(entity1, Velocity { dx: 0.5, dy: -0.5 });
        world.add_component(entity1, Health { value: 100 });

        // Entity missing Health
        let _entity2 = world.spawn_entity();
        world.add_component(_entity2, Position { x: 3.0, y: 4.0 });
        world.add_component(_entity2, Velocity { dx: 1.0, dy: 1.0 });

        // Query for entities with all three components
        let mut count = 0;
        for (pos, vel, health) in world.query::<(&mut Position, &Velocity, &Health), ()>() {
            count += 1;
            pos.x += vel.dx * (health.value as f32 / 100.0);
            pos.y += vel.dy * (health.value as f32 / 100.0);
        }

        // Should only find entity1
        assert_eq!(count, 1);

        // Verify the update occurred
        let pos1 = world.get_component::<Position>(entity1).unwrap();
        assert_eq!(pos1.x, 1.5); // 1.0 + 0.5 * 1.0
        assert_eq!(pos1.y, 1.5); // 2.0 + (-0.5) * 1.0
    }

    #[test]
    fn test_query_empty_results() {
        let mut world = World::new();

        // Create entities but none have both Position and Velocity
        let _entity1 = world.spawn_entity();
        world.add_component(_entity1, Position { x: 1.0, y: 2.0 });

        let _entity2 = world.spawn_entity();
        world.add_component(_entity2, Velocity { dx: 1.0, dy: 2.0 });

        // Query for entities with both components - should be empty
        let mut count = 0;
        for (_pos, _vel) in world.query::<(&Position, &Velocity), ()>() {
            count += 1;
        }
        assert_eq!(count, 0);

        // Query for non-existent component type should also be empty
        let mut health_count = 0;
        for _health in world.query::<&Health, ()>() {
            health_count += 1;
        }
        assert_eq!(health_count, 0);
    }

    #[test]
    fn test_query_iteration_count() {
        let mut world = World::new();

        // Create multiple entities with Position
        for i in 0..5 {
            let entity = world.spawn_entity();
            world.add_component(
                entity,
                Position {
                    x: i as f32,
                    y: (i * 2) as f32,
                },
            );
        }

        // Create entities with Velocity (some overlap, some don't)
        for i in 0..3 {
            let entity = Entity { index: i };
            world.add_component(
                entity,
                Velocity {
                    dx: i as f32,
                    dy: i as f32,
                },
            );
        }

        // Count Position queries
        let position_count = world.query::<&Position, ()>().into_iter().count();
        assert_eq!(position_count, 5);

        // Count Velocity queries
        let velocity_count = world.query::<&Velocity, ()>().into_iter().count();
        assert_eq!(velocity_count, 3);

        // Count combined queries (should be entities 0, 1, 2)
        let combined_count = world
            .query::<(&Position, &Velocity), ()>()
            .into_iter()
            .count();
        assert_eq!(combined_count, 3);
    }

    #[test]
    fn test_with_filter() {
        let mut world = World::new();

        // Entity with Position and Velocity
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });
        world.add_component(entity1, Velocity { dx: 0.5, dy: -0.5 });

        // Entity with Position only
        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 3.0, y: 4.0 });

        // Query Position with filter for entities that have Velocity
        let mut results = Vec::new();
        for pos in world.query::<&Position, With<Velocity>>() {
            results.push((pos.x, pos.y));
        }

        // Should only find entity1
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], (1.0, 2.0));
    }

    #[test]
    fn test_without_filter() {
        let mut world = World::new();

        // Entity with Position and Velocity
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });
        world.add_component(entity1, Velocity { dx: 0.5, dy: -0.5 });

        // Entity with Position only
        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 3.0, y: 4.0 });

        // Query Position with filter for entities that don't have Velocity
        let mut results = Vec::new();
        for pos in world.query::<&Position, Without<Velocity>>() {
            results.push((pos.x, pos.y));
        }

        // Should only find entity2
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], (3.0, 4.0));
    }

    #[test]
    fn test_multiple_filters() {
        let mut world = World::new();

        // Entity with Position, Velocity, and Health
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });
        world.add_component(entity1, Velocity { dx: 0.5, dy: -0.5 });
        world.add_component(entity1, Health { value: 100 });

        // Entity with Position and Velocity only
        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 3.0, y: 4.0 });
        world.add_component(entity2, Velocity { dx: 1.0, dy: 1.0 });

        // Entity with Position only
        let entity3 = world.spawn_entity();
        world.add_component(entity3, Position { x: 5.0, y: 6.0 });

        // Query Position with filter for entities that have Velocity but not Health
        let mut results = Vec::new();
        for pos in world.query::<&Position, (With<Velocity>, Without<Health>)>() {
            results.push((pos.x, pos.y));
        }

        // Should only find entity2
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], (3.0, 4.0));
    }

    // Simple system function - this approach works and is clean
    fn movement_system(query: Query<(&mut Position, &Velocity), (), World>) {
        for (pos, vel) in query {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }
    }

    fn damage_system(query: Query<&mut Health, With<Position>, World>) {
        for health in query {
            health.value -= 1;
        }
    }

    fn healing_system(query: Query<&mut Health, Without<Velocity>, World>) {
        for health in query {
            health.value += 5;
        }
    }

    #[test]
    fn test_simple_system() {
        let mut world = World::new();

        // Create entities
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 0.0, y: 0.0 });
        world.add_component(entity1, Velocity { dx: 1.0, dy: 2.0 });

        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 5.0, y: 5.0 });
        world.add_component(entity2, Velocity { dx: -1.0, dy: -2.0 });

        // Run movement system
        movement_system(world.query());

        // Verify positions were updated
        let pos1 = world.get_component::<Position>(entity1).unwrap();
        assert_eq!(pos1.x, 1.0);
        assert_eq!(pos1.y, 2.0);

        let pos2 = world.get_component::<Position>(entity2).unwrap();
        assert_eq!(pos2.x, 4.0);
        assert_eq!(pos2.y, 3.0);
    }

    #[test]
    fn test_filtered_system() {
        let mut world = World::new();

        // Entity with Position and Health
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 1.0, y: 2.0 });
        world.add_component(entity1, Health { value: 100 });

        // Entity with Health only
        let entity2 = world.spawn_entity();
        world.add_component(entity2, Health { value: 50 });

        // Run damage system (only affects entities with Position)
        damage_system(world.query());

        // Verify only entity1's health was reduced
        let health1 = world.get_component::<Health>(entity1).unwrap();
        assert_eq!(health1.value, 99);

        let health2 = world.get_component::<Health>(entity2).unwrap();
        assert_eq!(health2.value, 50);
    }

    fn generic_system<W: WorldTrait>(world: &mut W)
    where
        W: GetStorage<Position> + GetStorage<Velocity>,
    {
        let entity = world.spawn_entity();
        world.add_component(entity, Position { x: 5.0, y: 10.0 });
        world.add_component(entity, Velocity { dx: 1.0, dy: 2.0 });
    }

    #[test]
    fn test_world_trait_generic_function() {
        let mut world = World::new();

        generic_system(&mut world);

        let positions: Vec<_> = world.query::<&Position, ()>().into_iter().collect();
        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0].x, 5.0);
        assert_eq!(positions[0].y, 10.0);
    }

    #[test]
    fn test_multiple_systems() {
        let mut world = World::new();

        // Entity with Position, Velocity, and Health
        let entity1 = world.spawn_entity();
        world.add_component(entity1, Position { x: 0.0, y: 0.0 });
        world.add_component(entity1, Velocity { dx: 1.0, dy: 1.0 });
        world.add_component(entity1, Health { value: 100 });

        // Entity with Position and Health (no Velocity)
        let entity2 = world.spawn_entity();
        world.add_component(entity2, Position { x: 5.0, y: 5.0 });
        world.add_component(entity2, Health { value: 80 });

        // Run multiple systems
        movement_system(world.query());
        damage_system(world.query());
        healing_system(world.query());

        // Verify results
        let pos1 = world.get_component::<Position>(entity1).unwrap();
        assert_eq!(pos1.x, 1.0);
        assert_eq!(pos1.y, 1.0);

        let health1 = world.get_component::<Health>(entity1).unwrap();
        assert_eq!(health1.value, 99); // damaged but not healed (has velocity)

        let health2 = world.get_component::<Health>(entity2).unwrap();
        assert_eq!(health2.value, 84); // damaged and healed (no velocity)
    }
}
