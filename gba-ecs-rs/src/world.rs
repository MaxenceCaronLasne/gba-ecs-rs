use crate::{ComponentContainer, Entity, GetComponentContainer};

pub struct World<WC: WorldContainer> {
    last_entity: usize,
    containers: WC,
}

impl<WC: WorldContainer> World<WC> {
    pub fn new() -> Self {
        Self {
            last_entity: 0,
            containers: WC::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let entity = Entity::new(self.last_entity);
        self.last_entity += 1;
        self.containers.add_entity(entity);
        entity
    }

    pub fn add<C>(&mut self, entity: Entity, component: C)
    where
        WC: GetComponentContainer<C>,
    {
        self.containers.get_components_mut().set(entity, component);
    }

    pub fn get<C>(&self) -> &<WC as GetComponentContainer<C>>::Container
    where
        WC: GetComponentContainer<C>,
    {
        self.containers.get_components()
    }

    pub fn get_mut<C>(&mut self) -> &mut <WC as GetComponentContainer<C>>::Container
    where
        WC: GetComponentContainer<C>,
    {
        self.containers.get_components_mut()
    }
}

impl<WC: WorldContainer> Default for World<WC> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait WorldContainer {
    fn new() -> Self;
    fn add_entity(&mut self, entity: Entity);
}

// #[macro_export]
// macro_rules! world {
//     // New syntax: (Component, Allocator) pairs
//     ($name:ident { $(($component:ident, $allocator:path)),* $(,)? }) => {
//         #[allow(non_snake_case)]
//         struct $name {
//             last_entity: usize,
//             $(
//                 $component: $crate::VecComponentContainer<$component, $allocator>,
//             )*
//         }
//
//         impl $crate::World for $name {
//             fn new() -> Self {
//                 Self {
//                     last_entity: 0,
//                     $(
//                         $component: $crate::VecComponentContainer::new_in($allocator),
//                     )*
//                 }
//             }
//
//             fn add_entity(&mut self) -> $crate::Entity {
//                 let entity = $crate::Entity::new(self.last_entity);
//                 self.last_entity += 1;
//
//                 $(
//                     self.$component.add_entity(entity);
//                 )*
//
//                 entity
//             }
//
//             fn add_component<C>(&mut self, entity: $crate::Entity, component: C)
//             where
//                 Self: $crate::GetComponentContainer<C>,
//             {
//                 let container = self.get_components_mut::<C>();
//                 container.set(entity, component);
//             }
//         }
//
//         impl $name {
//             pub fn get_components<C>(&self) -> &<Self as $crate::GetComponentContainer<C>>::Container
//             where
//                 Self: $crate::GetComponentContainer<C>,
//             {
//                 $crate::GetComponentContainer::get_components(self)
//             }
//
//             pub fn get_components_mut<C>(&mut self) -> &mut <Self as $crate::GetComponentContainer<C>>::Container
//             where
//                 Self: $crate::GetComponentContainer<C>,
//             {
//                 $crate::GetComponentContainer::get_components_mut(self)
//             }
//         }
//
//         $(
//             impl $crate::GetComponentContainer<$component> for $name {
//                 type Container = $crate::VecComponentContainer<$component, $allocator>;
//
//                 fn get_components(&self) -> &Self::Container {
//                     &self.$component
//                 }
//
//                 fn get_components_mut(&mut self) -> &mut Self::Container {
//                     &mut self.$component
//                 }
//             }
//         )*
//     };
//
//     // Old syntax: just Component names (defaults to Global allocator)
//     ($name:ident { $($component:ident),* $(,)? }) => {
//         #[allow(non_snake_case)]
//         struct $name {
//             last_entity: usize,
//             $(
//                 $component: $crate::VecComponentContainer<$component>,
//             )*
//         }
//
//         impl $crate::World for $name {
//             fn new() -> Self {
//                 Self {
//                     last_entity: 0,
//                     $(
//                         $component: $crate::VecComponentContainer::new(),
//                     )*
//                 }
//             }
//
//             fn add_entity(&mut self) -> $crate::Entity {
//                 let entity = $crate::Entity::new(self.last_entity);
//                 self.last_entity += 1;
//
//                 $(
//                     self.$component.add_entity(entity);
//                 )*
//
//                 entity
//             }
//
//             fn add_component<C>(&mut self, entity: $crate::Entity, component: C)
//             where
//                 Self: $crate::GetComponentContainer<C>,
//             {
//                 let container = self.get_components_mut::<C>();
//                 container.set(entity, component);
//             }
//         }
//
//         impl $name {
//             pub fn get_components<C>(&self) -> &<Self as $crate::GetComponentContainer<C>>::Container
//             where
//                 Self: $crate::GetComponentContainer<C>,
//             {
//                 $crate::GetComponentContainer::get_components(self)
//             }
//
//             pub fn get_components_mut<C>(&mut self) -> &mut <Self as $crate::GetComponentContainer<C>>::Container
//             where
//                 Self: $crate::GetComponentContainer<C>,
//             {
//                 $crate::GetComponentContainer::get_components_mut(self)
//             }
//         }
//
//         $(
//             impl $crate::GetComponentContainer<$component> for $name {
//                 type Container = $crate::VecComponentContainer<$component>;
//
//                 fn get_components(&self) -> &Self::Container {
//                     &self.$component
//                 }
//
//                 fn get_components_mut(&mut self) -> &mut Self::Container {
//                     &mut self.$component
//                 }
//             }
//         )*
//     };
// }

