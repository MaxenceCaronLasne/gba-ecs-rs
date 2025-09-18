use super::filter::FilterQuery;
use super::item::ComponentQuery;
use crate::Entity;

pub struct QueryIterator<'w, C, F, W> {
    world: &'w mut W,
    current_entity: usize,
    max_entity: usize,
    filter: F,
    _phantom: core::marker::PhantomData<C>,
}

impl<'w, C, F, W> QueryIterator<'w, C, F, W> {
    pub fn new(world: &'w mut W, max_entity: usize, filter: F) -> Self {
        QueryIterator {
            world,
            current_entity: 0,
            max_entity,
            filter,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<'w, C, F, W> Iterator for QueryIterator<'w, C, F, W>
where
    C: ComponentQuery<'w, W>,
    F: FilterQuery<W>,
{
    type Item = C::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_entity < self.max_entity {
            let entity = Entity {
                index: self.current_entity,
            };
            self.current_entity += 1;

            // First check if the entity matches the filter
            if !self.filter.matches(self.world, entity) {
                continue;
            }

            unsafe {
                // We need unsafe here to get around the borrow checker
                // This is safe because we know each component storage is independent
                let world_ptr = self.world as *mut W;
                if let Some(item) = C::get_item_mut(&mut *world_ptr, entity) {
                    return Some(item);
                }
            }
        }
        None
    }
}
