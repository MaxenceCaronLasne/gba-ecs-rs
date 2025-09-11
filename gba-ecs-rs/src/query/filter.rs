use crate::{
    component::Component,
    storage::traits::{ComponentStorage, GetStorage},
    Entity,
};

pub trait Filter<W> {
    fn matches(&self, world: &W, entity_index: usize) -> bool;
}

pub struct With<T: Component>(core::marker::PhantomData<T>);

impl<T: Component> With<T> {
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl<T: Component> Default for With<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Component, W: GetStorage<T>> Filter<W> for With<T> {
    fn matches(&self, world: &W, entity_index: usize) -> bool {
        world
            .get_storage()
            .get(Entity {
                index: entity_index,
            })
            .is_some()
    }
}

pub struct Without<T: Component>(core::marker::PhantomData<T>);

impl<T: Component> Without<T> {
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl<T: Component> Default for Without<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Component, W: GetStorage<T>> Filter<W> for Without<T> {
    fn matches(&self, world: &W, entity_index: usize) -> bool {
        world
            .get_storage()
            .get(Entity {
                index: entity_index,
            })
            .is_none()
    }
}

impl<W> Filter<W> for () {
    fn matches(&self, _world: &W, _entity_index: usize) -> bool {
        true
    }
}

impl<F1, W> Filter<W> for (F1,)
where
    F1: Filter<W>,
{
    fn matches(&self, world: &W, entity_index: usize) -> bool {
        self.0.matches(world, entity_index)
    }
}

impl<F1, F2, W> Filter<W> for (F1, F2)
where
    F1: Filter<W>,
    F2: Filter<W>,
{
    fn matches(&self, world: &W, entity_index: usize) -> bool {
        self.0.matches(world, entity_index) && self.1.matches(world, entity_index)
    }
}

impl<F1, F2, F3, W> Filter<W> for (F1, F2, F3)
where
    F1: Filter<W>,
    F2: Filter<W>,
    F3: Filter<W>,
{
    fn matches(&self, world: &W, entity_index: usize) -> bool {
        self.0.matches(world, entity_index)
            && self.1.matches(world, entity_index)
            && self.2.matches(world, entity_index)
    }
}

impl<F1, F2, F3, F4, W> Filter<W> for (F1, F2, F3, F4)
where
    F1: Filter<W>,
    F2: Filter<W>,
    F3: Filter<W>,
    F4: Filter<W>,
{
    fn matches(&self, world: &W, entity_index: usize) -> bool {
        self.0.matches(world, entity_index)
            && self.1.matches(world, entity_index)
            && self.2.matches(world, entity_index)
            && self.3.matches(world, entity_index)
    }
}

