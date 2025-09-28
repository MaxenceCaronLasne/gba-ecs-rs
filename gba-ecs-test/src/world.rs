use crate::{Modulo1, Modulo2, Modulo8, Unique};
use agb::ExternalAllocator;
use gba_ecs_rs::{
    ComponentContainer, Entity, GetComponentContainer, VecComponentContainer, WorldContainer,
};

pub struct MyWorldContainer {
    modulo1: VecComponentContainer<Modulo1, ExternalAllocator>,
    modulo2: VecComponentContainer<Modulo2, ExternalAllocator>,
    modulo8: VecComponentContainer<Modulo8, ExternalAllocator>,
    unique: VecComponentContainer<Unique, ExternalAllocator>,
}

impl WorldContainer for MyWorldContainer {
    fn new() -> Self {
        Self {
            modulo1: VecComponentContainer::new_in(ExternalAllocator),
            modulo2: VecComponentContainer::new_in(ExternalAllocator),
            modulo8: VecComponentContainer::new_in(ExternalAllocator),
            unique: VecComponentContainer::new_in(ExternalAllocator),
        }
    }

    fn add_entity(&mut self, entity: Entity) {
        self.modulo1.add_entity(entity);
        self.modulo2.add_entity(entity);
        self.modulo8.add_entity(entity);
        self.unique.add_entity(entity);
    }
}

impl GetComponentContainer<Modulo1> for MyWorldContainer {
    type Container = VecComponentContainer<Modulo1, ExternalAllocator>;
    fn get_components(&self) -> &Self::Container {
        &self.modulo1
    }
    fn get_components_mut(&mut self) -> &mut Self::Container {
        &mut self.modulo1
    }
}

impl GetComponentContainer<Modulo2> for MyWorldContainer {
    type Container = VecComponentContainer<Modulo2, ExternalAllocator>;
    fn get_components(&self) -> &Self::Container {
        &self.modulo2
    }
    fn get_components_mut(&mut self) -> &mut Self::Container {
        &mut self.modulo2
    }
}

impl GetComponentContainer<Modulo8> for MyWorldContainer {
    type Container = VecComponentContainer<Modulo8, ExternalAllocator>;
    fn get_components(&self) -> &Self::Container {
        &self.modulo8
    }
    fn get_components_mut(&mut self) -> &mut Self::Container {
        &mut self.modulo8
    }
}

impl GetComponentContainer<Unique> for MyWorldContainer {
    type Container = VecComponentContainer<Unique, ExternalAllocator>;
    fn get_components(&self) -> &Self::Container {
        &self.unique
    }
    fn get_components_mut(&mut self) -> &mut Self::Container {
        &mut self.unique
    }
}
