use alloc::vec::Vec;
use core::marker::PhantomData;

use crate::VecComponentContainer;

pub struct ZippedQuery2<'a, T1, T2> {
    container1: *const Option<T1>,
    container2: *const Option<T2>,
    len: usize,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T1: 'a, T2: 'a> ZippedQuery2<'a, T1, T2> {
    fn new(container1: &'a Vec<Option<T1>>, container2: &'a Vec<Option<T2>>) -> Self {
        assert_eq!(container1.len(), container2.len());

        Self {
            container1: container1.as_ptr(),
            container2: container2.as_ptr(),
            len: container1.len(),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn for_each<F>(self, mut f: F)
    where
        F: FnMut(usize, &'a T1, &'a T2),
    {
        for i in 0..self.len {
            unsafe {
                let val1 = &*self.container1.add(i);
                let val2 = &*self.container2.add(i);

                if let Some(ref1) = val1 {
                    if let Some(ref2) = val2 {
                        f(i, ref1, ref2);
                    }
                }
            }
        }
    }

    #[inline]
    pub fn for_each_mut<F>(self, mut f: F)
    where
        F: FnMut(usize, &'a mut T1, &'a T2),
    {
        for i in 0..self.len {
            unsafe {
                let val1 = &mut *(self.container1 as *mut Option<T1>).add(i);
                let val2 = &*self.container2.add(i);

                if let Some(ref1) = val1 {
                    if let Some(ref2) = val2 {
                        f(i, ref1, ref2);
                    }
                }
            }
        }
    }
}

pub struct ZippedQuery3<'a, T1, T2, T3> {
    container1: *const Option<T1>,
    container2: *const Option<T2>,
    container3: *const Option<T3>,
    len: usize,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T1: 'a, T2: 'a, T3: 'a> ZippedQuery3<'a, T1, T2, T3> {
    fn new(
        container1: &'a Vec<Option<T1>>,
        container2: &'a Vec<Option<T2>>,
        container3: &'a Vec<Option<T3>>,
    ) -> Self {
        assert_eq!(container1.len(), container2.len());

        let len = container1.len().min(container2.len());
        Self {
            container1: container1.as_ptr(),
            container2: container2.as_ptr(),
            container3: container3.as_ptr(),
            len,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn for_each<F>(self, mut f: F)
    where
        F: FnMut(usize, &'a T1, &'a T2, &'a T3),
    {
        for i in 0..self.len {
            unsafe {
                let val1 = &*self.container1.add(i);
                let val2 = &*self.container2.add(i);
                let val3 = &*self.container3.add(i);

                if let Some(ref1) = val1 {
                    if let Some(ref2) = val2 {
                        if let Some(ref3) = val3 {
                            f(i, ref1, ref2, ref3);
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub fn for_each_mut<F>(self, mut f: F)
    where
        F: FnMut(usize, &'a mut T1, &'a T2, &'a T3),
    {
        for i in 0..self.len {
            unsafe {
                let val1 = &mut *(self.container1 as *mut Option<T1>).add(i);
                let val2 = &*self.container2.add(i);
                let val3 = &*self.container3.add(i);

                if let Some(ref1) = val1 {
                    if let Some(ref2) = val2 {
                        if let Some(ref3) = val3 {
                            f(i, ref1, ref2, ref3);
                        }
                    }
                }
            }
        }
    }
}

pub fn zip<'a, T1, T2>(
    container1: &'a VecComponentContainer<T1>,
    container2: &'a VecComponentContainer<T2>,
) -> ZippedQuery2<'a, T1, T2> {
    ZippedQuery2::new(&container1.container, &container2.container)
}

pub fn zip3<'a, T1, T2, T3>(
    container1: &'a VecComponentContainer<T1>,
    container2: &'a VecComponentContainer<T2>,
    container3: &'a VecComponentContainer<T3>,
) -> ZippedQuery3<'a, T1, T2, T3> {
    ZippedQuery3::new(
        &container1.container,
        &container2.container,
        &container3.container,
    )
}
