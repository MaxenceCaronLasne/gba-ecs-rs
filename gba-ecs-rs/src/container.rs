extern crate alloc;

use crate::Entity;
use alloc::alloc::Allocator;
use alloc::vec::Vec;

pub trait ComponentContainer<C> {
    fn add_entity(&mut self, entity: Entity);
    fn set(&mut self, entity: Entity, component: C);
    fn get(&self, entity: Entity) -> Option<&C>;
    fn get_index(&self, entity: usize) -> Option<&C>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut C>;
    fn get_index_mut(&mut self, entity: usize) -> Option<&mut C>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait GetComponentContainer<C> {
    type Container: ComponentContainer<C>;
    fn get_components(&self) -> &Self::Container;
    fn get_components_mut(&mut self) -> &mut Self::Container;
}

pub struct VecComponentContainer<C, A: Allocator = alloc::alloc::Global> {
    pub(crate) container: Vec<Option<C>, A>,
    pub(crate) active_indices: Vec<usize, A>,
}

impl<C> VecComponentContainer<C> {
    pub fn new() -> Self {
        Self {
            container: Vec::new(),
            active_indices: Vec::new(),
        }
    }
}

impl<C> Default for VecComponentContainer<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C, A: Allocator + Clone> VecComponentContainer<C, A> {
    pub fn new_in(allocator: A) -> Self {
        Self {
            container: Vec::new_in(allocator.clone()),
            active_indices: Vec::new_in(allocator),
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

    #[inline]
    pub fn for_each_sparse<F>(&self, mut f: F)
    where
        F: FnMut(usize, &C),
    {
        for &index in &self.active_indices {
            if let Some(Some(component)) = self.container.get(index) {
                f(index, component);
            }
        }
    }

    #[inline]
    pub fn for_each_sparse_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut C),
    {
        for &index in &self.active_indices {
            if let Some(Some(component)) = self.container.get_mut(index) {
                f(index, component);
            }
        }
    }
}

impl<C, A: Allocator + Clone> ComponentContainer<C> for VecComponentContainer<C, A> {
    fn add_entity(&mut self, entity: Entity) {
        while self.container.len() <= entity.index {
            self.container.push(None);
        }
    }

    fn get(&self, entity: Entity) -> Option<&C> {
        if let Some(Some(component)) = self.container.get(entity.index) {
            return Some(component);
        }

        None
    }

    fn get_index(&self, entity: usize) -> Option<&C> {
        if let Some(Some(component)) = self.container.get(entity) {
            return Some(component);
        }

        None
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        if let Some(Some(component)) = self.container.get_mut(entity.index) {
            return Some(component);
        }

        None
    }

    fn get_index_mut(&mut self, entity: usize) -> Option<&mut C> {
        if let Some(Some(component)) = self.container.get_mut(entity) {
            return Some(component);
        }

        None
    }

    fn set(&mut self, entity: Entity, component: C) {
        let index = entity.index;

        let is_new_component = self
            .container
            .get(index)
            .map(|opt| opt.is_none())
            .unwrap_or(true);

        self.container[index] = Some(component);

        if is_new_component {
            self.active_indices.push(index);
        }
    }

    fn len(&self) -> usize {
        self.container.len()
    }
}
