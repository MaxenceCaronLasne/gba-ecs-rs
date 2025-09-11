use super::item::ComponentQuery;
use super::filter::FilterQuery;
use super::iterator::QueryIterator;

/// Query struct for iterating over entities with specific components and filters.
pub struct Query<'w, C, F, W> {
    iterator: QueryIterator<'w, C, F, W>,
}

impl<'w, C, F, W> Query<'w, C, F, W>
where
    C: ComponentQuery<'w, W>,
    F: FilterQuery<W>,
{
    pub fn new(world: &'w mut W, max_entity: usize, filter: F) -> Self {
        Query {
            iterator: QueryIterator::new(world, max_entity, filter),
        }
    }
}

impl<'w, C, F, W> IntoIterator for Query<'w, C, F, W>
where
    C: ComponentQuery<'w, W>,
    F: FilterQuery<W>,
{
    type Item = C::Item;
    type IntoIter = QueryIterator<'w, C, F, W>;

    fn into_iter(self) -> Self::IntoIter {
        self.iterator
    }
}