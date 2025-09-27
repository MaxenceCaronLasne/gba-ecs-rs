extern crate alloc;

use crate::Entity;
use alloc::vec::Vec;

pub trait ComponentContainer<C> {
    fn add_entity(&mut self, entity: Entity);
    fn set(&mut self, entity: Entity, component: C);
    fn get(&self, entity: Entity) -> Option<&C>;
    fn get_index(&self, entity: usize) -> Option<&C>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut C>;
    fn get_index_mut(&mut self, entity: usize) -> Option<&mut C>;
    fn len(&self) -> usize;
}

pub trait GetComponentContainer<C> {
    type Container: ComponentContainer<C>;
    fn get_components(&self) -> &Self::Container;
    fn get_components_mut(&mut self) -> &mut Self::Container;
}

pub struct VecComponentContainer<C> {
    pub(crate) container: Vec<Option<C>>,
}

impl<C> VecComponentContainer<C> {
    pub fn new() -> Self {
        Self {
            container: Vec::new(),
        }
    }

    #[inline]
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(usize, &C),
    {
        let len = self.container.len();
        let ptr = self.container.as_ptr();

        for index in 0..len {
            unsafe {
                let val = &*ptr.add(index);
                if let Some(component) = val {
                    f(index, component);
                }
            }
        }
    }

    #[inline]
    pub fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut C),
    {
        let len = self.container.len();
        let ptr = self.container.as_mut_ptr();

        for index in 0..len {
            unsafe {
                let val = &mut *ptr.add(index);
                if let Some(component) = val {
                    f(index, component);
                }
            }
        }
    }
}

impl<C> ComponentContainer<C> for VecComponentContainer<C> {
    fn add_entity(&mut self, entity: Entity) {
        while self.container.len() <= entity.index {
            self.container.push(None);
        }
    }

    fn get(&self, entity: Entity) -> Option<&C> {
        if let Some(maybe_component) = self.container.get(entity.index) {
            if let Some(component) = maybe_component {
                return Some(component);
            }
        }

        return None;
    }

    fn get_index(&self, entity: usize) -> Option<&C> {
        if let Some(maybe_component) = self.container.get(entity) {
            if let Some(component) = maybe_component {
                return Some(component);
            }
        }

        return None;
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        if let Some(maybe_component) = self.container.get_mut(entity.index) {
            if let Some(component) = maybe_component {
                return Some(component);
            }
        }

        return None;
    }

    fn get_index_mut(&mut self, entity: usize) -> Option<&mut C> {
        if let Some(maybe_component) = self.container.get_mut(entity) {
            if let Some(component) = maybe_component {
                return Some(component);
            }
        }

        return None;
    }

    fn set(&mut self, entity: Entity, component: C) {
        self.container[entity.index] = Some(component);
    }

    fn len(&self) -> usize {
        self.container.len()
    }
}
