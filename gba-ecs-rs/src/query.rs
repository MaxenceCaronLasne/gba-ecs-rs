use crate::{GetComponentContainer, World, WorldContainer};

pub trait Query<'a> {
    type Item;

    fn for_each<WC, F>(world: &'a World<WC>, f: F)
    where
        WC: WorldContainer,
        F: FnMut(usize, Self::Item);
}

pub trait QueryMut<'a> {
    type Item;

    fn for_each_mut<WC, F>(world: &'a mut World<WC>, f: F)
    where
        WC: WorldContainer,
        F: FnMut(usize, Self::Item);
}

impl<'a, A: 'a, B: 'a> Query<'a> for (&A, &B) {
    type Item = (&'a A, &'a B);

    fn for_each<WC, F>(world: &'a World<WC>, f: F)
    where
        WC: WorldContainer,
        F: FnMut(usize, Self::Item),
    {
        todo!("Implement (&A, &B) query")
    }
}

impl<'a, A: 'a, B: 'a> QueryMut<'a> for (&mut A, &B) {
    type Item = (&'a mut A, &'a B);

    fn for_each_mut<WC, F>(world: &'a mut World<WC>, f: F)
    where
        WC: WorldContainer,
        F: FnMut(usize, Self::Item),
    {
        todo!("Implement (&mut A, &B) query")
    }
}

impl<'a, A: 'a, B: 'a> QueryMut<'a> for (&A, &mut B) {
    type Item = (&'a A, &'a mut B);

    fn for_each_mut<WC, F>(world: &'a mut World<WC>, f: F)
    where
        WC: WorldContainer,
        F: FnMut(usize, Self::Item),
    {
        todo!("Implement (&A, &mut B) query")
    }
}

impl<'a, A: 'a, B: 'a> QueryMut<'a> for (&mut A, &mut B) {
    type Item = (&'a mut A, &'a mut B);

    fn for_each_mut<WC, F>(world: &'a mut World<WC>, f: F)
    where
        WC: WorldContainer,
        F: FnMut(usize, Self::Item),
    {
        todo!("Implement (&mut A, &mut B) query")
    }
}
