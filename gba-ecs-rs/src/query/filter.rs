use crate::{
    component::Component,
    storage::traits::{ComponentStorage, GetStorage},
    Entity,
};

pub trait FilterQuery<W> {
    fn matches(&self, world: &W, entity: Entity) -> bool;
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

// FilterQuery implementations
impl<T: Component, W: GetStorage<T>> FilterQuery<W> for With<T> {
    fn matches(&self, world: &W, entity: Entity) -> bool {
        world.get_storage().get(entity).is_some()
    }
}

impl<T: Component, W: GetStorage<T>> FilterQuery<W> for Without<T> {
    fn matches(&self, world: &W, entity: Entity) -> bool {
        world.get_storage().get(entity).is_none()
    }
}

impl<W> FilterQuery<W> for () {
    fn matches(&self, _world: &W, _entity: Entity) -> bool {
        true
    }
}

// Tuple implementations for multiple filters
impl<F1, W> FilterQuery<W> for (F1,)
where
    F1: FilterQuery<W>,
{
    fn matches(&self, world: &W, entity: Entity) -> bool {
        self.0.matches(world, entity)
    }
}

impl<F1, F2, W> FilterQuery<W> for (F1, F2)
where
    F1: FilterQuery<W>,
    F2: FilterQuery<W>,
{
    fn matches(&self, world: &W, entity: Entity) -> bool {
        self.0.matches(world, entity) && self.1.matches(world, entity)
    }
}

impl<F1, F2, F3, W> FilterQuery<W> for (F1, F2, F3)
where
    F1: FilterQuery<W>,
    F2: FilterQuery<W>,
    F3: FilterQuery<W>,
{
    fn matches(&self, world: &W, entity: Entity) -> bool {
        self.0.matches(world, entity)
            && self.1.matches(world, entity)
            && self.2.matches(world, entity)
    }
}

impl<F1, F2, F3, F4, W> FilterQuery<W> for (F1, F2, F3, F4)
where
    F1: FilterQuery<W>,
    F2: FilterQuery<W>,
    F3: FilterQuery<W>,
    F4: FilterQuery<W>,
{
    fn matches(&self, world: &W, entity: Entity) -> bool {
        self.0.matches(world, entity)
            && self.1.matches(world, entity)
            && self.2.matches(world, entity)
            && self.3.matches(world, entity)
    }
}

// Trait for providing default instances of filter types
pub trait DefaultFilter {
    type Filter;
    fn default_filter() -> Self::Filter;
}

// Implement DefaultFilter for the filter types we want to support
impl<T: Component> DefaultFilter for With<T> {
    type Filter = With<T>;
    fn default_filter() -> Self::Filter {
        With::new()
    }
}

impl<T: Component> DefaultFilter for Without<T> {
    type Filter = Without<T>;
    fn default_filter() -> Self::Filter {
        Without::new()
    }
}

impl DefaultFilter for () {
    type Filter = ();
    fn default_filter() -> Self::Filter {
        ()
    }
}

// Support common tuple combinations
impl<F1, F2> DefaultFilter for (F1, F2)
where
    F1: DefaultFilter,
    F2: DefaultFilter,
{
    type Filter = (F1::Filter, F2::Filter);
    fn default_filter() -> Self::Filter {
        (F1::default_filter(), F2::default_filter())
    }
}

impl<F1, F2, F3> DefaultFilter for (F1, F2, F3)
where
    F1: DefaultFilter,
    F2: DefaultFilter,
    F3: DefaultFilter,
{
    type Filter = (F1::Filter, F2::Filter, F3::Filter);
    fn default_filter() -> Self::Filter {
        (
            F1::default_filter(),
            F2::default_filter(),
            F3::default_filter(),
        )
    }
}


