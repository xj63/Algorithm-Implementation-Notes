#![feature(test)]

use std::cmp::Ordering;

#[allow(dead_code)]
fn split_at<T>(slice: &[T], index: usize) -> (&[T], Option<&T>, &[T]) {
    if index >= slice.len() {
        return (&[], None, &[]);
    }
    (&slice[..index], Some(&slice[index]), &slice[index + 1..])
}

/// Splits a slice into three parts at the middle
///
/// [] -> (&[], None, &[])
/// [1] -> (&[], Some(&1), &[])
/// [1, 2] -> (&[1], Some(&2), &[])
/// [1, 2, 3] -> (&[1], Some(&2), &[3])
/// [1, 2, 3, 4] -> (&[1, 2], Some(&3), &[4])
/// [1, 2, 3, 4, 5] -> (&[1, 2], Some(&3), &[4, 5])
///
/// # Examples
///
/// ```
/// let slice = [1, 2, 3, 4, 5];
/// let (left, middle, right) = split_at_middle(&slice);
///
/// assert_eq!(left, &[1, 2]);
/// assert_eq!(middle, Some(&3));
/// assert_eq!(right, &[4, 5]);
/// ```
#[allow(dead_code)]
fn split_at_middle<T>(slice: &[T]) -> (&[T], Option<&T>, &[T]) {
    let mid = slice.len() / 2;
    split_at(slice, mid)
}

/// Searches for an element in a slice
///
/// Return None if not found
///        Some(first_index) if found
///
/// # Examples
///
/// ```
/// let slice = [1, 2, 3];
/// let index = sequential_search(&2, &[1, 2, 3]);
/// assert_eq!(index, Some(1));
/// ```
#[allow(dead_code)]
fn sequential_search<T: Eq>(target: &T, slice: &[T]) -> Option<usize> {
    slice
        .iter()
        .enumerate()
        .find_map(|(i, x)| if x == target { Some(i) } else { None })
}

/// Parallel searches for an element in a slice
///
/// Return None if not found
///        Some(index) if found
///
/// # Notice
///
/// This is not a parallel implementation.
/// index may not first find element.
///
/// # Examples
///
/// ```
/// let slice = [1, 2, 3];
/// let index = parallel_search(&2, &[1, 2, 3]);
/// assert_eq!(index, Some(1));
/// ```
#[allow(dead_code)]
fn parallel_search<T: Eq + Sync>(target: &T, slice: &[T]) -> Option<usize> {
    use rayon::prelude::*;

    slice
        .par_iter()
        .enumerate()
        .find_map_any(|(i, x)| if x == target { Some(i) } else { None })
}

/// std::binary_search for an element in a slice
#[allow(dead_code)]
fn std_binary_search<T: Ord>(target: &T, slice: &[T]) -> Option<usize> {
    match slice.binary_search(target) {
        Ok(index) => Some(index),
        Err(_) => None,
    }
}

/// binary_search for an element in a slice
///
/// 1 2 3 -> n monotonically increasing
#[allow(dead_code)]
fn binary_search<T: Ord>(target: &T, slice: &[T]) -> Option<usize> {
    let (left, middle, right) = split_at_middle(slice);
    Some(match Ord::cmp(target, middle?) {
        Ordering::Less => binary_search(target, left)?,
        Ordering::Equal => left.len(),
        Ordering::Greater => left.len() + 1 + binary_search(target, right)?,
    })
}

use num_traits::Num;

/// linear_search for an element in a slice
#[allow(dead_code)]
fn linear_search<T>(target: &T, slice: &[T]) -> Option<usize>
where
    T: Ord + Num + Into<f64> + Copy + Clone + std::fmt::Debug,
{
    let first = slice.first()?;
    let last = slice.last()?;

    if first == last {
        return if first == target { Some(0) } else { None };
    }

    let d: f64 = Into::<f64>::into(*last) - Into::<f64>::into(*first);
    let slope: f64 = d / (slice.len() - 1) as f64;
    let d: f64 = Into::<f64>::into(*target) - Into::<f64>::into(*first);
    let estimate_index = (d / slope) as usize;
    let estimate = slice.get(estimate_index)?;

    Some(match target.cmp(estimate) {
        Ordering::Less => linear_search(target, &slice[..estimate_index])?,
        Ordering::Equal => estimate_index,
        Ordering::Greater => {
            estimate_index + 1 + linear_search(target, &slice[estimate_index + 1..])?
        }
    })
}

fn main() {
    fn random_binary_search_sample(len: usize) -> (usize, Vec<u32>) {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        let check = rng.gen_range(0..len);
        let mut array: Vec<u32> = (0..len).map(|_| rng.gen()).collect();
        array.sort();

        (check, array)
    }

    let sample_1m = random_binary_search_sample(1_000);
    println!("{:?}", sample_1m.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod split_at_middle {
        use super::*;

        #[test]
        fn zero() {
            let slice: [usize; 0] = [];
            let (left, middle, right) = split_at_middle(&slice);
            assert_eq!(left, &[]);
            assert_eq!(middle, None);
            assert_eq!(right, &[]);
        }

        #[test]
        fn one() {
            let slice = [1];
            let (left, middle, right) = split_at_middle(&slice);
            assert_eq!(left, &[]);
            assert_eq!(middle, Some(&1));
            assert_eq!(right, &[]);
        }

        #[test]
        fn two() {
            let slice = [1, 2];
            let (left, middle, right) = split_at_middle(&slice);
            assert_eq!(left, &[1]);
            assert_eq!(middle, Some(&2));
            assert_eq!(right, &[]);
        }

        #[test]
        fn three() {
            let slice = [1, 2, 3];
            let (left, middle, right) = split_at_middle(&slice);
            assert_eq!(left, &[1]);
            assert_eq!(middle, Some(&2));
            assert_eq!(right, &[3]);
        }

        #[test]
        fn four() {
            let slice = [1, 2, 3, 4];
            let (left, middle, right) = split_at_middle(&slice);
            assert_eq!(left, &[1, 2]);
            assert_eq!(middle, Some(&3));
            assert_eq!(right, &[4]);
        }

        #[test]
        fn five() {
            let slice = [1, 2, 3, 4, 5];
            let (left, middle, right) = split_at_middle(&slice);
            assert_eq!(left, &[1, 2]);
            assert_eq!(middle, Some(&3));
            assert_eq!(right, &[4, 5]);
        }
    }

    mod sequential_search {
        use super::*;

        #[test]
        fn no_such_element() {
            let slice = [1, 2, 3];
            let index = sequential_search(&4, &slice);
            assert_eq!(index, None);
        }

        #[test]
        fn slice_is_empty() {
            let slice: [usize; 0] = [];
            let index = sequential_search(&1, &slice);
            assert_eq!(index, None);
        }

        #[test]
        fn find() {
            let slice = [1, 2, 3];
            let index = sequential_search(&2, &slice);
            assert_eq!(index, Some(1));
        }

        #[test]
        fn random_check() {
            use rand::{thread_rng, Rng};
            let mut rng = thread_rng();
            let len = 100;

            let array: Vec<usize> = (0..len).map(|_| rng.gen()).collect();
            let check = rng.gen_range(0..len);

            let index = sequential_search(&array[check], &array);
            assert_eq!(index, Some(check));
        }
    }

    mod parallel_search {
        use super::*;

        #[test]
        fn no_such_element() {
            let slice = [1, 2, 3];
            let index = parallel_search(&4, &slice);
            assert_eq!(index, None);
        }

        #[test]
        fn slice_is_empty() {
            let slice: [usize; 0] = [];
            let index = parallel_search(&1, &slice);
            assert_eq!(index, None);
        }

        #[test]
        fn find() {
            let slice = [1, 2, 3];
            let index = parallel_search(&2, &slice);
            assert_eq!(index, Some(1));
        }

        #[test]
        fn random_check() {
            use rand::{thread_rng, Rng};
            let mut rng = thread_rng();
            let len = 100;

            let array: Vec<usize> = (0..len).map(|_| rng.gen()).collect();
            let check = rng.gen_range(0..len);

            let index = parallel_search(&array[check], &array);
            assert_eq!(index, Some(check));
        }
    }

    mod binary_search {
        use super::*;

        #[test]
        fn no_such_element() {
            let slice = [1, 2, 3];
            let index = binary_search(&4, &slice);
            assert_eq!(index, None);
        }

        #[test]
        fn slice_is_empty() {
            let slice: [usize; 0] = [];
            let index = binary_search(&1, &slice);
            assert_eq!(index, None);
        }

        #[test]
        fn find() {
            let slice = [1, 2, 3];
            let index = binary_search(&2, &slice);
            assert_eq!(index, Some(1));
        }

        #[test]
        fn big_array() {
            let slice = [1, 2, 3, 4, 5];
            let index = binary_search(&1, &slice);
            assert_eq!(index, Some(0));
        }

        #[test]
        fn random_check() {
            use rand::{thread_rng, Rng};
            let mut rng = thread_rng();
            let len = 100;

            let mut array: Vec<usize> = (0..len).map(|_| rng.gen()).collect();
            let check = rng.gen_range(0..len);

            array.sort();
            let index = binary_search(&array[check], &array);
            assert_eq!(index, Some(check));
        }

        #[test]
        fn random_check_repeatedly() {
            for _ in 0..100 {
                random_check()
            }
        }
    }

    mod linear_search {
        use super::*;

        #[test]
        fn no_such_element() {
            let slice = [1, 2, 3];
            let index = linear_search(&4, &slice);
            assert_eq!(index, None);
        }

        #[test]
        fn slice_is_empty() {
            let slice: [i32; 0] = [];
            let index = linear_search(&1, &slice);
            assert_eq!(index, None);
        }

        #[test]
        fn find() {
            let slice = [1, 2, 3];
            let index = linear_search(&2, &slice);
            assert_eq!(index, Some(1));
        }

        #[test]
        fn big_array() {
            let slice = [1, 2, 3, 4, 5];
            let index = linear_search(&1, &slice);
            assert_eq!(index, Some(0));
        }

        #[test]
        fn random_check() {
            use rand::{thread_rng, Rng};
            let mut rng = thread_rng();
            let len = 100;

            let mut array: Vec<i32> = (0..len).map(|_| rng.gen()).collect();
            let check = rng.gen_range(0..len);

            array.sort();
            let index = linear_search(&array[check], &array);
            assert_eq!(index, Some(check));
        }

        #[test]
        fn random_check_repeatedly() {
            for _ in 0..100 {
                random_check()
            }
        }
    }

    mod benchs {
        extern crate test;
        use super::*;

        fn random_binary_search_sample(len: usize) -> (usize, Vec<u32>) {
            use rand::{thread_rng, Rng};
            let mut rng = thread_rng();

            let check = rng.gen_range(0..len);
            let mut array: Vec<u32> = (0..len).map(|_| rng.gen()).collect();
            array.sort();

            (check, array)
        }

        use lazy_static::lazy_static;
        lazy_static! {
            static ref SAMPLE_1K: (usize, Vec<u32>) = random_binary_search_sample(1000);
            static ref SAMPLE_1M: (usize, Vec<u32>) = random_binary_search_sample(1_000_000);
        }

        mod sequential {
            use super::*;

            #[bench]
            fn search_1k(b: &mut test::Bencher) {
                let (check, ref array) = *SAMPLE_1K;
                b.iter(|| sequential_search(&array[check], array));
            }

            #[bench]
            fn search_1m(b: &mut test::Bencher) {
                let (check, ref array) = *SAMPLE_1M;
                b.iter(|| sequential_search(&array[check], array));
            }

            #[bench]
            fn search_1m_last(b: &mut test::Bencher) {
                let (_, ref array) = *SAMPLE_1M;
                b.iter(|| sequential_search(array.last().unwrap(), array));
            }
        }

        mod parallel {
            use super::*;

            #[bench]
            fn search_1k(b: &mut test::Bencher) {
                let (check, ref array) = *SAMPLE_1K;
                b.iter(|| parallel_search(&array[check], array));
            }

            #[bench]
            fn search_1m(b: &mut test::Bencher) {
                let (check, ref array) = *SAMPLE_1M;
                b.iter(|| parallel_search(&array[check], array));
            }

            #[bench]
            fn search_1m_last(b: &mut test::Bencher) {
                let (_, ref array) = *SAMPLE_1M;
                b.iter(|| parallel_search(array.last().unwrap(), array));
            }
        }

        mod binary {
            use super::*;

            #[bench]
            fn search_1k(b: &mut test::Bencher) {
                let (check, ref array) = *SAMPLE_1K;
                b.iter(|| binary_search(&array[check], array));
            }

            #[bench]
            fn search_1m(b: &mut test::Bencher) {
                let (check, ref array) = *SAMPLE_1M;
                b.iter(|| binary_search(&array[check], array));
            }

            #[bench]
            fn search_1m_last(b: &mut test::Bencher) {
                let (_, ref array) = *SAMPLE_1M;
                b.iter(|| binary_search(array.last().unwrap(), array));
            }
        }

        mod std_binary {
            use super::*;

            #[bench]
            fn search_1k(b: &mut test::Bencher) {
                let (check, ref array) = *SAMPLE_1K;
                b.iter(|| std_binary_search(&array[check], array));
            }

            #[bench]
            fn search_1m(b: &mut test::Bencher) {
                let (check, ref array) = *SAMPLE_1M;
                b.iter(|| std_binary_search(&array[check], array));
            }

            #[bench]
            fn search_1m_last(b: &mut test::Bencher) {
                let (_, ref array) = *SAMPLE_1M;
                b.iter(|| std_binary_search(array.last().unwrap(), array));
            }
        }

        mod linear_search {
            use super::*;

            #[bench]
            fn search_1k(b: &mut test::Bencher) {
                let (check, ref array) = *SAMPLE_1K;
                b.iter(|| linear_search(&array[check], array));
            }

            #[bench]
            fn search_1m(b: &mut test::Bencher) {
                let (check, ref array) = *SAMPLE_1M;
                b.iter(|| linear_search(&array[check], array));
            }

            #[bench]
            fn search_1m_last(b: &mut test::Bencher) {
                let (_, ref array) = *SAMPLE_1M;
                b.iter(|| linear_search(array.last().unwrap(), array));
            }
        }
    }
}
