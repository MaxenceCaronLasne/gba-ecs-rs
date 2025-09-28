extern crate alloc;

use crate::Entity;

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

    fn for_each<F>(&self, f: F)
    where
        F: FnMut(usize, &C);

    fn for_each_mut<F>(&mut self, f: F)
    where
        F: FnMut(usize, &mut C);

    fn is_vec_container(&self) -> bool {
        false
    }
}

pub trait GetComponentContainer<C> {
    type Container: ComponentContainer<C>;
    fn get_components(&self) -> &Self::Container;
    fn get_components_mut(&mut self) -> &mut Self::Container;
}
