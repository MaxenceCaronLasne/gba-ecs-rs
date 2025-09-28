use agb::hash_map::HashMap;
use alloc::alloc::Allocator;
use alloc::alloc::Global;

pub struct HashComponentContainer<C, A: Allocator = Global> {
    container: HashMap<usize, C, A>,
}

impl<C> HashComponentContainer<C> {
    pub fn new() -> Self {
        Self {
            container: HashMap::new(),
        }
    }
}

impl<C> Default for HashComponentContainer<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C, A: Allocator + Clone> HashComponentContainer<C, A> {
    pub fn new_in(allocator: A) -> Self {
        Self {
            container: HashMap::new_in(allocator),
        }
    }

    #[inline]
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(usize, &C),
    {
        self.container.iter().for_each(|(index, component)| {
            f(*index, component);
        });
    }

    #[inline]
    pub fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut C),
    {
        self.container.iter_mut().for_each(|(index, component)| {
            f(*index, component);
        });
    }
}

impl<C, A: Allocator + Clone> crate::ComponentContainer<C> for HashComponentContainer<C, A> {
    #[inline]
    fn add_entity(&mut self, _entity: crate::Entity) {}

    #[inline]
    fn set(&mut self, entity: crate::Entity, component: C) {
        self.container.insert(entity.index, component);
    }

    #[inline]
    fn get(&self, entity: crate::Entity) -> Option<&C> {
        let index = entity.index;
        self.container.get(&index)
    }

    #[inline]
    fn get_index(&self, entity: usize) -> Option<&C> {
        self.container.get(&entity)
    }

    #[inline]
    fn get_mut(&mut self, entity: crate::Entity) -> Option<&mut C> {
        let index = entity.index;
        self.container.get_mut(&index)
    }

    #[inline]
    fn get_index_mut(&mut self, entity: usize) -> Option<&mut C> {
        self.container.get_mut(&entity)
    }

    #[inline]
    fn len(&self) -> usize {
        self.container.len()
    }

    fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(usize, &C),
    {
        self.container.iter().for_each(|(index, component)| {
            f(*index, component);
        });
    }

    fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut C),
    {
        self.container.iter_mut().for_each(|(index, component)| {
            f(*index, component);
        });
    }
}
