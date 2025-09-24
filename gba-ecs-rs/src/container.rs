extern crate alloc;

use crate::Entity;
use alloc::vec::Vec;

pub trait ComponentContainer<C> {
    fn add_entity(&mut self, entity: Entity);
    fn set(&mut self, entity: Entity, component: C);
    fn get(&self, entity: Entity) -> Option<&C>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut C>;
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(usize, &C);
    fn for_each_mut<F>(&mut self, f: F)
    where
        F: FnMut(usize, &mut C);
    fn is_sparse(&self) -> bool;
}

pub trait GetComponentContainer<C> {
    type Container: ComponentContainer<C>;
    fn get_components(&self) -> &Self::Container;
    fn get_components_mut(&mut self) -> &mut Self::Container;
}

pub struct DenseComponentContainer<C> {
    pub container: Vec<Option<C>>,
}

pub struct SparseComponentContainer<C> {
    container: agb::hash_map::HashMap<usize, C>,
}

pub trait MarkerContainer {
    fn add_entity(&mut self, entity: Entity);
    fn set(&mut self, entity: Entity);
    fn is_present(&self, entity: Entity) -> bool;
    fn iter(&self) -> impl Iterator<Item = usize> + '_;
}

pub struct DenseMarkerContainer {
    container: Vec<bool>,
}

pub struct SparseMarkerContainer {
    container: agb::hash_map::HashSet<usize>,
}

impl DenseMarkerContainer {
    pub fn new() -> Self {
        Self {
            container: Vec::new(),
        }
    }
}

impl SparseMarkerContainer {
    pub fn new() -> Self {
        Self {
            container: agb::hash_map::HashSet::new(),
        }
    }
}

impl MarkerContainer for SparseMarkerContainer {
    fn add_entity(&mut self, entity: Entity) {}

    fn set(&mut self, entity: Entity) {
        _ = self.container.insert(entity.index)
    }

    fn is_present(&self, entity: Entity) -> bool {
        self.container.contains(&entity.index)
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.container.iter().copied()
    }
}

impl MarkerContainer for DenseMarkerContainer {
    fn set(&mut self, entity: Entity) {
        self.container[entity.index] = true;
    }

    fn add_entity(&mut self, entity: Entity) {
        while self.container.len() <= entity.index {
            self.container.push(false);
        }
    }

    fn is_present(&self, entity: Entity) -> bool {
        if let Some(res) = self.container.get(entity.index) {
            return *res;
        }

        false
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.container
            .iter()
            .enumerate()
            .filter(|(_i, b)| **b)
            .map(|(i, _b)| i)
    }
}

impl<C> DenseComponentContainer<C> {
    pub fn new() -> Self {
        Self {
            container: Vec::new(),
        }
    }

}

impl<C> ComponentContainer<C> for DenseComponentContainer<C> {
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

    fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        if let Some(maybe_component) = self.container.get_mut(entity.index) {
            if let Some(component) = maybe_component {
                return Some(component);
            }
        }

        return None;
    }

    fn set(&mut self, entity: Entity, component: C) {
        self.container[entity.index] = Some(component);
    }

    #[inline]
    fn for_each<F>(&self, mut f: F)
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
    fn for_each_mut<F>(&mut self, mut f: F)
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
    fn is_sparse(&self) -> bool {
        false
    }
}

impl<C> SparseComponentContainer<C> {
    pub fn new() -> Self {
        Self {
            container: agb::hash_map::HashMap::new(),
        }
    }
}

impl<C> ComponentContainer<C> for SparseComponentContainer<C> {
    fn add_entity(&mut self, entity: Entity) {}

    fn get(&self, entity: Entity) -> Option<&C> {
        self.container.get(&entity.index)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.container.get_mut(&entity.index)
    }

    fn set(&mut self, entity: Entity, component: C) {
        self.container.insert(entity.index, component);
    }

    #[inline]
    fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(usize, &C),
    {
        for (&index, component) in &self.container {
            f(index, component);
        }
    }

    #[inline]
    fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut C),
    {
        for (&index, component) in self.container.iter_mut() {
            f(index, component);
        }
    }

    fn is_sparse(&self) -> bool {
        true
    }
}
