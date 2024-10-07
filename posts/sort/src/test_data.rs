use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::{thread_rng, Rng};
use std::mem::MaybeUninit;
use std::ops::AddAssign;

/// Generates an array of random values in the given range.
///
/// # Examples
///
/// ```
/// use sort::test_data::gen_random;
/// let range = -5..10;
/// let array: [i8; 100] = gen_random(range.clone());
/// assert!(array.iter().all(|x| range.contains(x)));
/// ```
pub fn gen_random<T, R, const N: usize>(range: R) -> [T; N]
where
    T: SampleUniform,
    R: SampleRange<T> + Clone,
{
    let mut rng = thread_rng();
    let uninit_array = MaybeUninit::<[T; N]>::uninit();

    unsafe {
        let mut array = uninit_array.assume_init();
        array
            .iter_mut()
            .for_each(|x| *x = rng.gen_range(range.clone()));
        array
    }
}

/// Random walk starting from start with `stride`
///
/// generates an array starting from `start`
/// and incrementing by random values within the `stride` range.
///
/// # Examples
///
/// ```
/// use sort::test_data::gen_stroll;
/// let range = 0..10;
/// let array: [isize; 100] = gen_stroll(0, range.clone());
/// for i in 1..array.len() {
///     assert!(range.contains(&(array[i] - array[i - 1])));
/// }
/// ```
pub fn gen_stroll<T, R, const N: usize>(start: T, stride: R) -> [T; N]
where
    T: SampleUniform + AddAssign + Copy,
    R: SampleRange<T> + Clone,
{
    let mut array = gen_random(stride);

    if N == 0 {
        return array;
    }

    array[0] = start;
    for i in 1..N {
        array[i] += array[i - 1];
    }

    array
}

/// Generates an array of values using the given function
///
/// # Examples
///
/// ```
/// use sort::test_data::gen_function;
/// let f = |x| (3 * x + 10) as isize;
/// let array: [isize; 100] = gen_function(f);
/// assert!(array.iter().enumerate().all(|(i, &x)| x == f(i)));
/// ```
pub fn gen_function<T, const N: usize>(function: impl FnMut(usize) -> T) -> [T; N] {
    let mut f = function;
    let uninit_array = MaybeUninit::<[T; N]>::uninit();

    unsafe {
        let mut array = uninit_array.assume_init();
        array.iter_mut().enumerate().for_each(|(i, x)| *x = f(i));
        array
    }
}

/// Merges two arrays by adding their corresponding elements
///
/// # Examples
///
/// ```
/// use sort::test_data::merge_array;
/// let array = merge_array([1, 2, 3], [4, 5, 6]);
/// assert_eq!(array, [5, 7, 9]);
/// ```
pub fn merge_array<T: AddAssign, const N: usize>(lhs: [T; N], rhs: [T; N]) -> [T; N] {
    let mut lhs = lhs;
    lhs.iter_mut().zip(rhs).for_each(|(lhs, rhs)| *lhs += rhs);
    lhs
}

/// Apply random noise to an array
///
/// # Examples
///
/// ```
/// use sort::test_data::with_noise;
/// let array = with_noise([0, 1, 2], 3..5);
/// assert!((3..5).contains(&array[0]));
/// assert!((4..6).contains(&array[1]));
/// assert!((5..7).contains(&array[2]));
/// ```
pub fn with_noise<T, R, const N: usize>(array: [T; N], noise: R) -> [T; N]
where
    T: SampleUniform + AddAssign,
    R: SampleRange<T> + Clone,
{
    let noise = gen_random(noise);
    merge_array(array, noise)
}

#[cfg(test)]
mod test_gen {
    use super::*;
    use crate::read_bench_data;

    fn is_function<T: Eq>(slice: &[T], function: impl FnMut(usize) -> T) -> bool {
        let mut f = function;
        slice.iter().enumerate().all(|(i, x)| *x == f(i))
    }

    #[test]
    fn read_bench_data() {
        let _: [i32; 10000] = read_bench_data!("random");
        let _: [i32; 10000] = read_bench_data!("stroll");
        let _: [i32; 1000] = read_bench_data!("trend-increasing");
        let _: [i32; 1000] = read_bench_data!("gaussian-with-noise");
        let _: [i32; 1000] = read_bench_data!("high-sample-sin-with-noise");
        let _: [i32; 1000] = read_bench_data!("low-sample-sin-with-noise");
    }

    mod random {
        use super::*;

        #[test]
        fn empty() {
            let array: [isize; 0] = gen_random(0..10);
            assert!(array.is_empty());
        }

        #[test]
        fn in_range() {
            let range = -5..10;
            let array: [i8; 100] = gen_random(range.clone());
            assert!(array.iter().all(|x| range.contains(x)));
        }
    }

    mod stroll {
        use super::*;

        #[test]
        fn empty() {
            let array: [usize; 0] = gen_stroll(0, 0..10);
            assert!(array.is_empty());
        }

        #[test]
        fn monotonically_increasing() {
            let range = 0..10;
            let array: [isize; 100] = gen_stroll(0, range.clone());
            for i in 1..array.len() {
                assert!(array[i] >= array[i - 1]);
            }
        }

        #[test]
        fn monotonically_decreasing() {
            let range = -10..=0;
            let array: [isize; 100] = gen_stroll(0, range.clone());
            for i in 1..array.len() {
                assert!(array[i] <= array[i - 1]);
            }
        }

        #[test]
        fn stride_range() {
            let range = -5..=5;
            let array: [isize; 100] = gen_stroll(0, range.clone());
            for i in 1..array.len() {
                assert!(range.contains(&(array[i] - array[i - 1])));
            }
        }

        #[test]
        fn first_element() {
            let range = 0..10;
            let array: [isize; 3] = gen_stroll(5, range.clone());
            assert_eq!(array[0], 5);
        }
    }

    mod function {
        use super::*;

        #[test]
        fn empty() {
            let array: [isize; 0] = gen_function(|_| 0);
            assert!(array.is_empty());
        }

        #[test]
        fn zero() {
            let f = |_| 0;
            let array: [isize; 10] = gen_function(f);
            assert!(is_function(&array, f));
        }

        #[test]
        fn constant() {
            let c = 10;
            let f = |_| c;
            let array: [isize; 10] = gen_function(f);
            assert!(is_function(&array, f));
        }

        #[test]
        fn linear() {
            let k = -1;
            let b = 5;
            let f = |x| k * x as isize + b;
            let array: [isize; 100] = gen_function(f);
            assert!(is_function(&array, f));
        }

        #[test]
        fn quadratic() {
            let f = |x| {
                let x = x as i32;
                (3 * x - 10) * (x + 7)
            };
            let array: [i32; 100] = gen_function(f);
            assert!(is_function(&array, f));
        }

        #[test]
        fn sin() {
            let f = |x| (100.0 * (x as f64).sin()) as i16;
            let array: [i16; 100] = gen_function(f);
            assert!(is_function(&array, f));
        }

        #[test]
        fn gaussian() {
            let mean = 50.0;
            let std_dev = 10.0;
            let f = |x| {
                let x = x as f64;
                (100.0 * (-((x - mean) * (x - mean)) / (2.0 * std_dev * std_dev)).exp()) as isize
            };
            let array: [isize; 100] = gen_function(f);
            assert!(is_function(&array, f));
        }
    }

    mod merge {
        use super::*;

        #[test]
        fn empty() {
            let lhs: [isize; 0] = [];
            let rhs: [isize; 0] = [];
            let array = merge_array(lhs, rhs);
            assert!(array.is_empty());
        }

        #[test]
        fn sin_add_linear() {
            let sin = |x| (100.0 * (x as f64).sin()) as isize;
            let linear = |x| 3 * x as isize - 10;
            let combine = |x| sin(x) + linear(x);

            let array_sin: [isize; 100] = gen_function(sin);
            let array_linear: [isize; 100] = gen_function(linear);

            let array = merge_array(array_sin, array_linear);
            assert!(is_function(&array, combine));
        }
    }

    mod with_noise {
        use super::*;

        #[test]
        fn noise_in_range() {
            let f = |x| 3 * x as isize + 5;
            let noise = -5..=10;
            let array: [isize; 100] = with_noise(gen_function(f), noise.clone());
            assert!(array
                .into_iter()
                .enumerate()
                .map(|(i, x)| x - f(i))
                .all(|x| noise.contains(&x)));
        }
    }
}
