#[macro_export]
macro_rules! read_bench_data {
    ($path:literal) => {
        match include_str!(concat!("../bench-data/", $path, ".json")) {
            s => {
                let vec: Vec<i32> = serde_json::from_str(s).unwrap();
                vec.try_into().unwrap()
            }
        }
    };
}

use crate::Solution;
use std::cmp::Ordering;

pub fn test_sort<T, const N: usize, S>(_: S, array: [T; N]) -> bool
where
    T: Ord + Copy,
    S: Solution<T, N>,
{
    let sorted = S::sort(array);
    let mut array = array;
    array.sort();
    *sorted == array
}

pub fn test_sort_by<T, const N: usize, S, F>(_: S, array: [T; N], cmp: F) -> bool
where
    T: Ord + Copy,
    S: Solution<T, N>,
    F: Fn(&T, &T) -> Ordering,
{
    let sorted = S::sort_by(array, |a, b| cmp(a, b));
    let mut array = array;
    array.sort_by(|a, b| cmp(a, b));
    *sorted == array
}

pub fn test_sort_by_key<T, const N: usize, S, F, K>(_: S, array: [T; N], f: F) -> bool
where
    T: Eq + Copy,
    S: Solution<T, N>,
    F: FnMut(&T) -> K,
    K: Ord,
{
    let mut f = f;
    let sorted = S::sort_by_key(array, |a| f(a)).take();
    let mut array = array;
    array.sort_by_key(|a| f(a));
    sorted == array
}

pub fn test_empty(solution: impl Solution<i32, 0>) {
    let data = [];
    assert!(test_sort(solution, data));
    assert!(test_sort_by(solution, data, |a, b| a.cmp(b)));
    assert!(test_sort_by_key(solution, data, |i| *i));
}

pub fn test_simple(solution: impl Solution<i32, 10>) {
    let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert!(test_sort(solution, data));
    assert!(test_sort_by(solution, data, |a, b| a.cmp(b)));
    assert!(test_sort_by_key(solution, data, |i| *i));
}

pub fn test_random(solution: impl Solution<i32, 10_000>) {
    let data = read_bench_data!("random");
    assert!(test_sort(solution, data));
    assert!(test_sort_by(solution, data, |a, b| a.cmp(b)));
    assert!(test_sort_by_key(solution, data, |i| *i));
}

pub fn test_stroll(solution: impl Solution<i32, 10_000>) {
    let data = read_bench_data!("stroll");
    assert!(test_sort(solution, data));
    assert!(test_sort_by(solution, data, |a, b| a.cmp(b)));
    assert!(test_sort_by_key(solution, data, |i| *i));
}

pub fn test_trend_increasing(solution: impl Solution<i32, 1000>) {
    let data = read_bench_data!("trend-increasing");
    assert!(test_sort(solution, data));
    assert!(test_sort_by(solution, data, |a, b| a.cmp(b)));
    assert!(test_sort_by_key(solution, data, |i| *i));
}

pub fn test_gaussian_with_noise(solution: impl Solution<i32, 1000>) {
    let data = read_bench_data!("gaussian-with-noise");
    assert!(test_sort(solution, data));
    assert!(test_sort_by(solution, data, |a, b| a.cmp(b)));
    assert!(test_sort_by_key(solution, data, |i| *i));
}

pub fn test_low_sample_sin_with_noise(solution: impl Solution<i32, 1000>) {
    let data = read_bench_data!("low-sample-sin-with-noise");
    assert!(test_sort(solution, data));
    assert!(test_sort_by(solution, data, |a, b| a.cmp(b)));
    assert!(test_sort_by_key(solution, data, |i| *i));
}

pub fn test_high_sample_sin_with_noise(solution: impl Solution<i32, 1000>) {
    let data = read_bench_data!("high-sample-sin-with-noise");
    assert!(test_sort(solution, data));
    assert!(test_sort_by(solution, data, |a, b| a.cmp(b)));
    assert!(test_sort_by_key(solution, data, |i| *i));
}
