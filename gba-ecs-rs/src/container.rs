extern crate alloc;

use crate::Entity;
use alloc::vec::Vec;

pub trait Component {
    fn prev(&self) -> Option<usize>;
    fn next(&self) -> Option<usize>;
    fn set_prev(&mut self, prev: Option<usize>);
    fn set_next(&mut self, next: Option<usize>);
}

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
    head: Option<usize>,
}

impl<C> VecComponentContainer<C> {
    pub fn new() -> Self {
        Self {
            container: Vec::new(),
            head: None,
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
        C: Component,
    {
        let mut current = self.head;
        while let Some(index) = current {
            if let Some(Some(component)) = self.container.get(index) {
                f(index, component);
                current = component.next();
            } else {
                break;
            }
        }
    }

    #[inline]
    pub fn for_each_sparse_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut C),
        C: Component,
    {
        let mut current = self.head;
        while let Some(index) = current {
            if let Some(Some(component)) = self.container.get_mut(index) {
                let next = component.next();
                f(index, component);
                current = next;
            } else {
                break;
            }
        }
    }
}

impl<C: Component> ComponentContainer<C> for VecComponentContainer<C> {
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

    fn set(&mut self, entity: Entity, mut component: C) {
        let index = entity.index;

        component.set_prev(None);
        component.set_next(self.head);

        if let Some(old_head) = self.head {
            if let Some(Some(old_head_component)) = self.container.get_mut(old_head) {
                old_head_component.set_prev(Some(index));
            }
        }

        self.container[index] = Some(component);
        self.head = Some(index);
    }

    fn len(&self) -> usize {
        self.container.len()
    }
}
