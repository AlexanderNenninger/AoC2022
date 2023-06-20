#![allow(unused)]
use std::{
    ops::{Index, IndexMut},
    ptr,
};

/// Heap allocated array with static size `N`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeapArray<const N: usize, T> {
    data: Box<[T]>,
    len: usize,
}

impl<const N: usize, T> HeapArray<N, T> {
    pub fn new() -> Self {
        let data = unsafe {
            Box::<[T]>::try_new_uninit_slice(N)
                .expect("ERROR: Allocation error.")
                .assume_init()
        };
        Self { data, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, item: T) {
        if self.len >= N {
            panic!("ERROR: StaticVec out of Capacity.")
        }
        unsafe {
            let item_ptr = self.data.get_unchecked_mut(self.len);
            ptr::write_volatile(item_ptr, item);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len <= 0 {
            return None;
        }
        self.len -= 1;
        Some(unsafe { self.data.as_ptr().add(self.len).read() })
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &T> + '_ {
        self.data.iter().take(self.len)
    }
}

impl<const N: usize, T> Index<usize> for HeapArray<N, T> {
    type Output = T;

    #[inline(always)]
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        if index < self.len {
            return self.data.index(index);
        }
        panic!("Index out of bounds with static vec of size {N}")
    }
}

impl<const N: usize, T> IndexMut<usize> for HeapArray<N, T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        if index < self.len {
            return self.data.index_mut(index);
        }
        panic!("Index out of bounds with static vec of size {N}")
    }
}

#[cfg(test)]
mod test {
    use super::HeapArray;

    #[test]
    fn test_push_pop() {
        let mut v = HeapArray::<5, _>::new();
        assert_eq!(v.data.len(), 5);
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(5);

        assert_eq!(v.pop().unwrap(), 5);
        assert_eq!(v.pop().unwrap(), 4);
        assert_eq!(v.pop().unwrap(), 3);
        assert_eq!(v.pop().unwrap(), 2);
        assert_eq!(v.pop().unwrap(), 1);
    }

    #[test]
    #[should_panic]
    fn out_of_capacity() {
        let mut v = HeapArray::<1, _>::new();
        v.push(0);
        v.push(1);
    }

    #[test]
    fn modify() {
        let mut v = HeapArray::<2, _>::new();
        v.push(0);
        v.push(1);
        v[1] = 0;
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let v = HeapArray::<1, usize>::new();
        v[0];
    }

    #[test]
    #[should_panic]
    fn index_mut_out_of_bounds() {
        struct Test {
            u: usize,
        }
        let mut v = HeapArray::<1, Test>::new();
        let u = &mut v[0];
    }

    #[test]
    fn iter() {
        let mut v = HeapArray::<4, _>::new();
        v.push(0);
        v.push(1);

        for (i, item) in v.iter().enumerate() {
            assert_eq!(i, *item);
        }
    }
}
