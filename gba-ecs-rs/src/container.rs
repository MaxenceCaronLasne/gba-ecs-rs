extern crate alloc;

use crate::Entity;
use alloc::vec::Vec;

pub trait ComponentContainer<C> {
    fn add_entity(&mut self, entity: Entity);
    fn set(&mut self, entity: Entity, component: C);
    fn get(&self, entity: Entity) -> Option<&C>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut C>;
    fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, &'a C)> + 'a
    where
        C: 'a;
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (usize, &'a mut C)> + 'a
    where
        C: 'a;
    fn is_sparse(&self) -> bool;
}

pub trait GetComponentContainer<C> {
    type Container: ComponentContainer<C>;
    fn get_components(&self) -> &Self::Container;
    fn get_components_mut(&mut self) -> &mut Self::Container;
}

pub struct DenseComponentContainer<C> {
    container: Vec<Option<C>>,
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

    fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, &'a C)> + 'a
    where
        C: 'a,
    {
        self.container
            .iter()
            .enumerate()
            .filter_map(|(i, o)| match o {
                Some(value) => Some((i, value)),
                None => None,
            })
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (usize, &'a mut C)> + 'a
    where
        C: 'a,
    {
        self.container
            .iter_mut()
            .enumerate()
            .filter_map(|(i, o)| match o {
                Some(value) => Some((i, value)),
                None => None,
            })
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

    fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, &'a C)> + 'a
    where
        C: 'a,
    {
        self.container.iter().map(|(i, c)| (*i, c))
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (usize, &'a mut C)> + 'a
    where
        C: 'a,
    {
        self.container.iter_mut().map(|(i, c)| (*i, c))
    }

    fn is_sparse(&self) -> bool {
        true
    }
}
