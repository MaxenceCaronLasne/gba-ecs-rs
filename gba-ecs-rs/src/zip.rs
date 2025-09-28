use alloc::alloc::Allocator;
use core::marker::PhantomData;

use crate::VecComponentContainer;

pub struct ZippedQuery2<'a, T1, T2> {
    container1: *const Option<T1>,
    container2: *const Option<T2>,
    len: usize,
    shortest_active_indices: *const usize,
    shortest_active_len: usize,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T1: 'a, T2: 'a> ZippedQuery2<'a, T1, T2> {
    fn new<A1: Allocator + Clone, A2: Allocator + Clone>(
        container1_full: &'a VecComponentContainer<T1, A1>,
        container2_full: &'a VecComponentContainer<T2, A2>,
    ) -> Self {
        let container1 = &container1_full.container;
        let container2 = &container2_full.container;
        assert_eq!(container1.len(), container2.len());

        // Find the shortest active_indices vector for sparse iteration optimization
        let (shortest_active_indices, shortest_active_len) =
            if container1_full.active_indices.len() <= container2_full.active_indices.len() {
                (
                    container1_full.active_indices.as_ptr(),
                    container1_full.active_indices.len(),
                )
            } else {
                (
                    container2_full.active_indices.as_ptr(),
                    container2_full.active_indices.len(),
                )
            };

        Self {
            container1: container1.as_ptr(),
            container2: container2.as_ptr(),
            len: container1.len(),
            shortest_active_indices,
            shortest_active_len,
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

    #[inline]
    pub fn for_each_sparse_mut<F>(self, mut f: F)
    where
        F: FnMut(usize, &'a mut T1, &'a T2),
    {
        for i in 0..self.shortest_active_len {
            let index = unsafe { *self.shortest_active_indices.add(i) };
            unsafe {
                let val1 = &mut *(self.container1 as *mut Option<T1>).add(index);
                let val2 = &*self.container2.add(index);

                if let Some(ref1) = val1 {
                    if let Some(ref2) = val2 {
                        f(index, ref1, ref2);
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
    shortest_active_indices: *const usize,
    shortest_active_len: usize,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T1: 'a, T2: 'a, T3: 'a> ZippedQuery3<'a, T1, T2, T3> {
    fn new<A1: Allocator + Clone, A2: Allocator + Clone, A3: Allocator + Clone>(
        container1_full: &'a VecComponentContainer<T1, A1>,
        container2_full: &'a VecComponentContainer<T2, A2>,
        container3_full: &'a VecComponentContainer<T3, A3>,
    ) -> Self {
        let container1 = &container1_full.container;
        let container2 = &container2_full.container;
        let container3 = &container3_full.container;
        assert_eq!(container1.len(), container2.len());

        // Find the shortest active_indices vector for sparse iteration optimization
        let (shortest_active_indices, shortest_active_len) = if container1_full.active_indices.len()
            <= container2_full.active_indices.len()
            && container1_full.active_indices.len() <= container3_full.active_indices.len()
        {
            (
                container1_full.active_indices.as_ptr(),
                container1_full.active_indices.len(),
            )
        } else if container2_full.active_indices.len() <= container3_full.active_indices.len() {
            (
                container2_full.active_indices.as_ptr(),
                container2_full.active_indices.len(),
            )
        } else {
            (
                container3_full.active_indices.as_ptr(),
                container3_full.active_indices.len(),
            )
        };

        let len = container1.len().min(container2.len());
        Self {
            container1: container1.as_ptr(),
            container2: container2.as_ptr(),
            container3: container3.as_ptr(),
            len,
            shortest_active_indices,
            shortest_active_len,
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

    #[inline]
    pub fn for_each_sparse_mut<F>(self, mut f: F)
    where
        F: FnMut(usize, &'a mut T1, &'a T2, &'a T3),
    {
        for i in 0..self.shortest_active_len {
            let index = unsafe { *self.shortest_active_indices.add(i) };
            unsafe {
                let val1 = &mut *(self.container1 as *mut Option<T1>).add(index);
                let val2 = &*self.container2.add(index);
                let val3 = &*self.container3.add(index);

                if let Some(ref1) = val1 {
                    if let Some(ref2) = val2 {
                        if let Some(ref3) = val3 {
                            f(index, ref1, ref2, ref3);
                        }
                    }
                }
            }
        }
    }
}

pub fn zip<'a, T1, T2, A1: Allocator + Clone, A2: Allocator + Clone>(
    container1: &'a VecComponentContainer<T1, A1>,
    container2: &'a VecComponentContainer<T2, A2>,
) -> ZippedQuery2<'a, T1, T2> {
    ZippedQuery2::new(container1, container2)
}

pub fn zip3<'a, T1, T2, T3, A1: Allocator + Clone, A2: Allocator + Clone, A3: Allocator + Clone>(
    container1: &'a VecComponentContainer<T1, A1>,
    container2: &'a VecComponentContainer<T2, A2>,
    container3: &'a VecComponentContainer<T3, A3>,
) -> ZippedQuery3<'a, T1, T2, T3> {
    ZippedQuery3::new(container1, container2, container3)
}
