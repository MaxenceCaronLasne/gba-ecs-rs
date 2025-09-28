use crate::{ComponentContainer, Entity, GetComponentContainer, World, WorldContainer};

pub trait Query<'a, WC: WorldContainer> {
    type Item;

    fn for_each<F>(world: &'a World<WC>, f: F)
    where
        F: FnMut(usize, Self::Item);
}

impl<'a, A: 'a, B: 'a, WC> Query<'a, WC> for (&A, &B)
where
    WC: WorldContainer + GetComponentContainer<A> + GetComponentContainer<B>,
    <WC as GetComponentContainer<A>>::Container: ComponentContainer<A>,
    <WC as GetComponentContainer<B>>::Container: ComponentContainer<B>,
{
    type Item = (&'a A, &'a B);

    fn for_each<F>(world: &'a World<WC>, mut f: F)
    where
        F: FnMut(usize, Self::Item),
    {
        let container_a = world.get::<A>();
        let container_b = world.get::<B>();

        // Use unsafe to extend lifetime - this is safe because the world reference
        // lives for 'a and the components are borrowed from it
        unsafe {
            let container_a_ptr = container_a as *const <WC as GetComponentContainer<A>>::Container;
            let container_b_ptr = container_b as *const <WC as GetComponentContainer<B>>::Container;

            (*container_a_ptr).for_each(|entity_index, component_a| {
                let entity = Entity::new(entity_index);
                if let Some(component_b) = (*container_b_ptr).get(entity) {
                    let component_a_extended: &'a A = core::mem::transmute(component_a);
                    let component_b_extended: &'a B = core::mem::transmute(component_b);
                    f(entity_index, (component_a_extended, component_b_extended));
                }
            });
        }
    }
}
