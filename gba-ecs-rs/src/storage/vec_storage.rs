extern crate alloc;
use super::traits::ComponentStorage;
use crate::{Component, Entity};
use alloc::vec::Vec;

pub struct VecStorage<C: Component> {
    components: Vec<Option<C>>,
}

impl<C: Component> ComponentStorage<C> for VecStorage<C> {
    fn new() -> Self {
        VecStorage {
            components: Vec::new(),
        }
    }

    fn get(&self, entity: Entity) -> Option<&C> {
        self.components.get(entity.index)?.as_ref()
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.components.get_mut(entity.index)?.as_mut()
    }

    fn insert(&mut self, entity: Entity, component: C) {
        self.ensure_capacity(entity);
        self.components[entity.index] = Some(component);
    }

    fn remove(&mut self, entity: Entity) -> Option<C> {
        if entity.index < self.components.len() {
            self.components[entity.index].take()
        } else {
            None
        }
    }

    fn ensure_capacity(&mut self, entity: Entity) {
        if entity.index >= self.components.len() {
            self.components.resize_with(entity.index + 1, || None);
        }
    }
}

