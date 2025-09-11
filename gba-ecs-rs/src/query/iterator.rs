use super::item::{QueryItem, QueryItemWithFilter};
use super::filter::Filter;
use crate::Entity;

pub struct QueryIterator<'w, Q, W> {
    world: &'w mut W,
    current_entity: usize,
    max_entity: usize,
    _phantom: core::marker::PhantomData<Q>,
}

impl<'w, Q, W> QueryIterator<'w, Q, W> {
    pub fn new(world: &'w mut W, max_entity: usize) -> Self {
        QueryIterator {
            world,
            current_entity: 0,
            max_entity,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<'w, Q: QueryItem<'w, W>, W> Iterator for QueryIterator<'w, Q, W> {
    type Item = Q::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_entity < self.max_entity {
            let entity = Entity {
                index: self.current_entity,
            };
            self.current_entity += 1;

            unsafe {
                // We need unsafe here to get around the borrow checker
                // This is safe because we know each component storage is independent
                let world_ptr = self.world as *mut W;
                if let Some(item) = Q::get_item_mut(&mut *world_ptr, entity) {
                    return Some(item);
                }
            }
        }
        None
    }
}

pub struct FilteredQueryIterator<'w, Q, F, W> {
    world: &'w mut W,
    current_entity: usize,
    max_entity: usize,
    filter: F,
    _phantom: core::marker::PhantomData<Q>,
}

impl<'w, Q, F, W> FilteredQueryIterator<'w, Q, F, W> {
    pub fn new(world: &'w mut W, max_entity: usize, filter: F) -> Self {
        FilteredQueryIterator {
            world,
            current_entity: 0,
            max_entity,
            filter,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<'w, Q, F, W> Iterator for FilteredQueryIterator<'w, Q, F, W>
where
    Q: QueryItemWithFilter<'w, W, F>,
    F: Filter<W>,
{
    type Item = Q::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_entity < self.max_entity {
            let entity = Entity {
                index: self.current_entity,
            };
            self.current_entity += 1;

            unsafe {
                // We need unsafe here to get around the borrow checker
                // This is safe because we know each component storage is independent
                let world_ptr = self.world as *mut W;
                if let Some(item) = Q::get_item_mut_filtered(&mut *world_ptr, entity, &self.filter) {
                    return Some(item);
                }
            }
        }
        None
    }
}

