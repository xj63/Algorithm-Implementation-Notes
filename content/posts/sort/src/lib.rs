#![feature(generic_const_exprs)]
#![feature(concat_idents)]
#![feature(test)]
#![allow(dead_code)]

extern crate openmp_sys;

pub mod test_data;
#[cfg(test)]
pub(crate) mod test_utils;

pub mod csort;

pub mod merge_two_sorted;

mod stable;

pub use stable::Stable;

use std::cmp::{Ord, Ordering};

pub trait Solution<T, const N: usize>: Copy + Clone {
    fn sort_method<F>(array: [T; N], is_less: F) -> Sorted<T, N, impl FnMut(&T, &T) -> bool>
    where
        F: FnMut(&T, &T) -> bool;

    fn sort(array: [T; N]) -> Sorted<T, N, impl FnMut(&T, &T) -> bool>
    where
        T: Ord,
    {
        Self::sort_method(array, T::lt)
    }

    fn sort_by<F>(array: [T; N], mut compare: F) -> Sorted<T, N, impl FnMut(&T, &T) -> bool>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        Self::sort_method(array, move |a, b| compare(a, b) == Ordering::Less)
    }

    fn sort_by_key<F, K>(array: [T; N], mut key: F) -> Sorted<T, N, impl FnMut(&T, &T) -> bool>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        Self::sort_method(array, move |a, b| key(a) < key(b))
    }
}

use std::cell::RefCell;

pub struct Sorted<T, const N: usize, F: FnMut(&T, &T) -> bool> {
    array: [T; N],
    rule_is_less: RefCell<F>,
}

impl<T, const N: usize, F: FnMut(&T, &T) -> bool> Sorted<T, N, F> {
    pub fn take(self) -> [T; N] {
        self.array
    }

    /// # Safety
    ///
    /// The caller must ensure that the array is sorted.
    pub unsafe fn uncheck_from_array(array: [T; N], rule_is_less: F) -> Self {
        Sorted {
            array,
            rule_is_less: RefCell::new(rule_is_less),
        }
    }

    pub fn force_check(&self) -> bool {
        self.array
            .windows(2)
            .all(|w| (self.rule_is_less.borrow_mut())(&w[0], &w[1]))
    }
}

impl<T, const N: usize, F> std::borrow::Borrow<[T; N]> for Sorted<T, N, F>
where
    F: FnMut(&T, &T) -> bool,
{
    fn borrow(&self) -> &[T; N] {
        &self.array
    }
}

impl<T, const N: usize, F> std::convert::AsRef<[T; N]> for Sorted<T, N, F>
where
    F: FnMut(&T, &T) -> bool,
{
    fn as_ref(&self) -> &[T; N] {
        &self.array
    }
}

impl<T, const N: usize, F> std::ops::Deref for Sorted<T, N, F>
where
    F: FnMut(&T, &T) -> bool,
{
    type Target = [T; N];
    fn deref(&self) -> &Self::Target {
        &self.array
    }
}
