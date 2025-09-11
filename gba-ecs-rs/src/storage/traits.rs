use crate::{Component, Entity};

pub trait GetStorage<C: Component> {
    type Storage: ComponentStorage<C>;
    fn get_storage(&self) -> &Self::Storage;
    fn get_storage_mut(&mut self) -> &mut Self::Storage;
}

pub trait ComponentStorage<C: Component> {
    fn new() -> Self;
    fn get(&self, entity: Entity) -> Option<&C>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut C>;
    fn insert(&mut self, entity: Entity, component: C);
    fn remove(&mut self, entity: Entity) -> Option<C>;
    fn ensure_capacity(&mut self, entity: Entity);
}

