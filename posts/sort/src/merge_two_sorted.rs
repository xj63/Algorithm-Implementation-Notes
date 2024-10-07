use std::mem::MaybeUninit;

struct Stack<T, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T, const N: usize> Stack<T, N> {
    fn new() -> Self {
        let data = MaybeUninit::uninit();
        Self {
            data: unsafe { data.assume_init() },
            len: 0,
        }
    }

    fn push(&mut self, element: T) {
        if self.len == N {
            panic!("Stack is full");
        }

        self.data[self.len] = element;
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        use std::ptr;
        match self.len {
            0 => None,
            _ => unsafe {
                self.len -= 1;
                core::hint::assert_unchecked(self.len < N);
                Some(ptr::read(self.data.as_ptr().add(self.len)))
            },
        }
    }

    fn take_if_full(self) -> Option<[T; N]> {
        if self.len == N {
            Some(self.data)
        } else {
            None
        }
    }

    unsafe fn take_no_check(self) -> [T; N] {
        core::hint::assert_unchecked(self.len == N);
        self.data
    }
}

use std::iter::{IntoIterator, Iterator};

struct StackIterator<T, const N: usize> {
    stack: Stack<T, N>,
}

impl<T, const N: usize> Iterator for StackIterator<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T, const N: usize> IntoIterator for Stack<T, N> {
    type Item = T;
    type IntoIter = StackIterator<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        StackIterator { stack: self }
    }
}

/// Merge two sorted arrays in place
///
/// # Examples
///
/// ```
/// #![feature(generic_const_exprs)]
/// use sort::merge_two_sorted::merge_two_sorted_rs;
/// let array1 = [1, 3, 5];
/// let array2 = [2, 9];
/// let array3 = merge_two_sorted_rs(array1, array2);
/// assert_eq!(&array3, &[1, 2, 3, 5, 9]);
/// ```
pub fn merge_two_sorted_rs<T, const N: usize, const M: usize>(a: [T; N], b: [T; M]) -> [T; N + M]
where
    T: Ord + Copy,
{
    let mut stack = Stack::new();

    let mut b = b.into_iter();
    let mut i = 0;
    while i < N {
        match b.next() {
            Some(x) => {
                while i < N && a[i] <= x {
                    stack.push(a[i]);
                    i += 1;
                }
                stack.push(x);
            }
            None => break,
        }
    }

    a.into_iter().skip(i).for_each(|x| stack.push(x));
    b.for_each(|x| stack.push(x));

    stack.take_if_full().unwrap()
}

pub fn merge_two_sorted(array: &mut [i32], split: usize) {
    use crate::csort::merge_two_sorted_array;

    merge_two_sorted_array(array, split);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_element_array() {
        let mut array = [5, 2];
        merge_two_sorted(&mut array, 1);
        assert_eq!(array, [2, 5]);
    }

    #[test]
    fn empty_element_right() {
        let mut array = [0];
        merge_two_sorted(&mut array, 0);
        assert_eq!(array, [0]);
    }

    #[test]
    fn empty_element_left() {
        let mut array = [2];
        merge_two_sorted(&mut array, 1);
        assert_eq!(array, [2]);
    }

    #[test]
    fn empty_element_both() {
        let mut array: [i32; 0] = [];
        merge_two_sorted(&mut array, 0);
        assert_eq!(array, [0; 0]);
    }

    #[test]
    fn merge_array() {
        let mut array = [-1, 0, 3, 5, 6, -3, 5, 7, 8, 8, 9, 10];
        merge_two_sorted(&mut array, 5);
        assert_eq!(array, [-3, -1, 0, 3, 5, 5, 6, 7, 8, 8, 9, 10]);
    }

    mod bench {
        extern crate test;
        use super::*;
        use crate::test_data::*;

        #[bench]
        fn array1k(b: &mut test::Bencher) {
            let mut array: [i32; 1000] = gen_random(-1000..1000);
            let (array1, array2) = array.split_at_mut(500);
            array1.sort();
            array2.sort();

            b.iter(|| merge_two_sorted(&mut array, 500))
        }
    }
}
