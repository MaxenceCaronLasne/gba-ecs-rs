use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced, parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated, Ident,
    Token,
};

/// Generates a complete ECS World struct with storage for the specified components.
///
/// This macro creates a world struct that can manage entities and their components.
/// For each component type listed, it generates:
/// - A storage field to hold component data
/// - Implementation of `GetStorage` trait for type-safe component access
/// - Methods for entity and component management
///
/// # Syntax
///
/// ```rust,ignore
/// define_world!(WorldName {
///     Component1,
///     Component2,
///     // ... more components
/// });
/// ```
///
/// # Generated Methods
///
/// The generated world struct includes these public methods:
/// - `new()` - Creates a new empty world
/// - `spawn_entity()` - Creates a new entity and returns its ID
/// - `add_component(entity, component)` - Adds a component to an entity
/// - `remove_component::<T>(entity)` - Removes and returns a component from an entity
/// - `get_component::<T>(entity)` - Gets a read-only reference to a component
/// - `get_component_mut::<T>(entity)` - Gets a mutable reference to a component
///
/// # Examples
///
/// ```rust,ignore
/// use gba_ecs_macros::{Component, define_world};
///
/// #[derive(Component)]
/// struct Position { x: f32, y: f32 }
///
/// #[derive(Component)]
/// struct Velocity { dx: f32, dy: f32 }
///
/// // Generate the world struct
/// define_world!(GameWorld {
///     Position,
///     Velocity,
/// });
///
/// // Usage
/// let mut world = GameWorld::new();
/// let player = world.spawn_entity();
///
/// world.add_component(player, Position { x: 0.0, y: 0.0 });
/// world.add_component(player, Velocity { dx: 1.0, dy: 0.0 });
///
/// // Update position based on velocity
/// if let (Some(pos), Some(vel)) = (
///     world.get_component_mut::<Position>(player),
///     world.get_component::<Velocity>(player)
/// ) {
///     pos.x += vel.dx;
///     pos.y += vel.dy;
/// }
/// ```
///
/// # Implementation Details
///
/// - Each component gets its own `VecStorage<Component>` field
/// - Storage fields are named using the component name in snake_case with "_storage" suffix
/// - All storage operations are type-safe through the `GetStorage` trait
/// - Entity IDs are simple incrementing integers starting from 0
pub fn define_world_impl(input: TokenStream) -> TokenStream {
    // Parse the input syntax: WorldName { Component1, Component2, ... }
    struct WorldDefinition {
        world_name: Ident,
        components: Punctuated<Ident, Token![,]>,
    }

    impl Parse for WorldDefinition {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let world_name = input.parse()?;
            let content;
            braced!(content in input);
            let components = content.parse_terminated(Ident::parse, Token![,])?;

            Ok(WorldDefinition {
                world_name,
                components,
            })
        }
    }

    let world_def = parse_macro_input!(input as WorldDefinition);
    let world_name = &world_def.world_name;
    let components: Vec<_> = world_def.components.into_iter().collect();

    // Helper function to generate consistent storage field names
    let storage_field_name = |comp: &Ident| {
        let comp_name = comp.to_string();
        // Convert CamelCase to snake_case for field names
        let snake_case = comp_name
            .chars()
            .enumerate()
            .fold(String::new(), |mut acc, (i, c)| {
                if c.is_uppercase() && i > 0 {
                    acc.push('_');
                }
                acc.push(c.to_lowercase().next().unwrap_or(c));
                acc
            });
        quote::format_ident!("{}_storage", snake_case)
    };

    // Generate storage field declarations
    let storage_fields = components.iter().map(|comp| {
        let field_name = storage_field_name(comp);
        quote! {
            #field_name: gba_ecs_rs::VecStorage<#comp>
        }
    });

    // Generate GetStorage trait implementations for each component
    let get_storage_impls = components.iter().map(|comp| {
        let field_name = storage_field_name(comp);
        quote! {
            impl gba_ecs_rs::GetStorage<#comp> for #world_name {
                type Storage = gba_ecs_rs::VecStorage<#comp>;

                fn get_storage(&self) -> &Self::Storage {
                    &self.#field_name
                }

                fn get_storage_mut(&mut self) -> &mut Self::Storage {
                    &mut self.#field_name
                }
            }
        }
    });

    // Generate field initialization for the new() method
    let new_field_inits = components.iter().map(|comp| {
        let field_name = storage_field_name(comp);
        quote! {
            #field_name: gba_ecs_rs::ComponentStorage::new()
        }
    });

    // Generate the complete world struct and implementation
    let expanded = quote! {
        pub struct #world_name {
            #(#storage_fields,)*
            entity_count: usize,
        }

        // Implement GetStorage for each component type
        #(#get_storage_impls)*

        impl #world_name {
            /// Creates a new empty world with no entities or components.
            pub fn new() -> Self {
                #world_name {
                    #(#new_field_inits,)*
                    entity_count: 0,
                }
            }

            /// Spawns a new entity and returns its unique identifier.
            ///
            /// Each entity gets a unique ID that can be used to attach components.
            pub fn spawn_entity(&mut self) -> gba_ecs_rs::Entity {
                let entity_id = self.entity_count;
                self.entity_count += 1;
                gba_ecs_rs::Entity { index: entity_id }
            }

            /// Adds a component to the specified entity.
            ///
            /// If the entity already has a component of this type, it will be replaced.
            pub fn add_component<C>(&mut self, entity: gba_ecs_rs::Entity, component: C)
            where
                C: gba_ecs_rs::Component,
                Self: gba_ecs_rs::GetStorage<C>,
            {
                let storage = self.get_storage_mut();
                storage.insert(entity, component);
            }

            /// Removes a component from the specified entity and returns it.
            ///
            /// Returns `None` if the entity doesn't have a component of this type.
            pub fn remove_component<C>(&mut self, entity: gba_ecs_rs::Entity) -> Option<C>
            where
                C: gba_ecs_rs::Component,
                Self: gba_ecs_rs::GetStorage<C>,
            {
                let storage = self.get_storage_mut();
                storage.remove(entity)
            }

            /// Gets a read-only reference to a component on the specified entity.
            ///
            /// Returns `None` if the entity doesn't have a component of this type.
            pub fn get_component<C>(&self, entity: gba_ecs_rs::Entity) -> Option<&C>
            where
                C: gba_ecs_rs::Component,
                Self: gba_ecs_rs::GetStorage<C>,
            {
                let storage = self.get_storage();
                storage.get(entity)
            }

            /// Gets a mutable reference to a component on the specified entity.
            ///
            /// Returns `None` if the entity doesn't have a component of this type.
            pub fn get_component_mut<C>(&mut self, entity: gba_ecs_rs::Entity) -> Option<&mut C>
            where
                C: gba_ecs_rs::Component,
                Self: gba_ecs_rs::GetStorage<C>,
            {
                let storage = self.get_storage_mut();
                storage.get_mut(entity)
            }

            /// Query for entities with specific components.
            ///
            /// Returns an iterator that yields tuples of component references for entities
            /// that have all the required components.
            ///
            /// # Example
            /// ```rust,ignore
            /// // Query for entities with Position and Velocity components
            /// for (pos, vel) in world.query::<(&mut Position, &Velocity)>() {
            ///     pos.x += vel.dx;
            ///     pos.y += vel.dy;
            /// }
            /// ```
            pub fn query<Q>(&mut self) -> gba_ecs_rs::QueryIterator<Q, Self>
            where
                Q: for<'w> gba_ecs_rs::QueryItem<'w, Self>,
            {
                gba_ecs_rs::QueryIterator::new(self, self.entity_count)
            }

            /// Query for entities with specific components and filters.
            ///
            /// Returns a filtered iterator that yields tuples of component references for entities
            /// that have all the required components and match the filter criteria.
            ///
            /// # Example
            /// ```rust,ignore
            /// use gba_ecs_rs::{With, Without};
            /// 
            /// // Query for entities with Position, filtering for those with Velocity but without Health
            /// for pos in world.query_filtered::<&mut Position, (With<Velocity>, Without<Health>)>(
            ///     (With::new(), Without::new())
            /// ) {
            ///     pos.x += 1.0;
            /// }
            /// ```
            pub fn query_filtered<Q, F>(&mut self, filter: F) -> gba_ecs_rs::FilteredQueryIterator<Q, F, Self>
            where
                Q: for<'w> gba_ecs_rs::QueryItemWithFilter<'w, Self, F>,
                F: gba_ecs_rs::Filter<Self>,
            {
                gba_ecs_rs::FilteredQueryIterator::new(self, self.entity_count, filter)
            }
        }
    };

    TokenStream::from(expanded)
}

