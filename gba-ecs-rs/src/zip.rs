use alloc::vec::Vec;
use core::marker::PhantomData;

pub struct ZippedQuery<'a, T1, T2> {
    container1: *const Option<T1>,
    container2: *const Option<T2>,
    current_index: usize,
    len: usize,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T1: 'a, T2: 'a> ZippedQuery<'a, T1, T2> {
    pub fn new(container1: &'a Vec<Option<T1>>, container2: &'a Vec<Option<T2>>) -> Self {
        assert_eq!(container1.len(), container2.len());

        let len = container1.len().min(container2.len());
        Self {
            container1: container1.as_ptr(),
            container2: container2.as_ptr(),
            current_index: 0,
            len,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn for_each<F>(mut self, mut f: F)
    where
        F: FnMut(usize, &'a T1, &'a T2),
    {
        while self.current_index < self.len {
            let index = self.current_index;
            self.current_index += 1;

            unsafe {
                let val1 = &*self.container1.add(index);
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
