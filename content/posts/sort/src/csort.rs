#[path = "./csort_bind.rs"]
mod csort_bind;
use csort_bind as cbind;

pub fn merge_two_sorted_array(array: &mut [i32], split: usize) {
    if split > array.len() {
        panic!("split index out of bounds");
    }

    unsafe { cbind::merge_two_sorted_array(array.len() as u32, array.as_mut_ptr(), split as u32) }
}

pub fn bubble_sort(array: &mut [i32]) {
    unsafe { cbind::bubble_sort(array.len() as u32, array.as_mut_ptr()) }
}

pub fn selection_sort(array: &mut [i32]) {
    unsafe { cbind::selection_sort(array.len() as u32, array.as_mut_ptr()) }
}

pub fn insertion_sort(array: &mut [i32]) {
    unsafe { cbind::insertion_sort(array.len() as u32, array.as_mut_ptr()) }
}

pub fn merge_sort(array: &mut [i32]) {
    unsafe { cbind::merge_sort(array.len() as u32, array.as_mut_ptr()) }
}

pub fn merge_sort_parallel(array: &mut [i32]) {
    unsafe { cbind::merge_sort_parallel(array.len() as u32, array.as_mut_ptr()) }
}

pub fn radix_lsd_sort(array: &mut [i32]) {
    unsafe { cbind::radix_lsd_sort(array.len() as u32, array.as_mut_ptr()) }
}

pub fn cstd_qsort(array: &mut [i32]) {
    unsafe { cbind::std_qsort(array.len() as u32, array.as_mut_ptr()) }
}

pub fn quick_sort(array: &mut [i32]) {
    unsafe { cbind::quick_sort(array.len() as u32, array.as_mut_ptr()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod utils {
        use crate::read_bench_data;

        fn test_empty(solution: fn(&mut [i32])) {
            let mut data = [];
            solution(&mut data);
            assert_eq!(data, [] as [i32; 0]);
        }

        fn test_one(solution: fn(&mut [i32])) {
            let mut data = [1];
            solution(&mut data);
            assert_eq!(data, [1]);
        }

        fn test_two(solution: fn(&mut [i32])) {
            let mut data = [2, 1];
            solution(&mut data);
            assert_eq!(data, [1, 2]);
        }

        fn test_three(solution: fn(&mut [i32])) {
            let mut data = [3, 1, 2];
            solution(&mut data);
            assert_eq!(data, [1, 2, 3]);
        }

        fn test_four(solution: fn(&mut [i32])) {
            let mut data = [2, 3, 2, 1];
            solution(&mut data);
            assert_eq!(data, [1, 2, 2, 3]);
        }

        fn test_five(solution: fn(&mut [i32])) {
            let mut data = [5, 1, 4, 2, 3];
            solution(&mut data);
            assert_eq!(data, [1, 2, 3, 4, 5]);
        }

        fn test_origin(solution: fn(&mut [i32])) {
            let mut data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let mut sort = data;
            data.sort();
            solution(&mut sort);
            assert_eq!(sort, data);
        }

        fn test_simple(solution: fn(&mut [i32])) {
            let mut data = [9, 3, 5, 1, 7, 4, 6, 2, 8, 0];
            let mut sort = data;
            data.sort();
            solution(&mut sort);
            assert_eq!(sort, data);
        }

        fn test_random(solution: fn(&mut [i32])) {
            let mut data: [i32; 10000] = read_bench_data!("random");
            let mut sort = data;
            solution(&mut sort);
            data.sort();
            assert_eq!(sort, data);
        }

        fn test_stroll(solution: fn(&mut [i32])) {
            let mut data: [i32; 10000] = read_bench_data!("stroll");
            let mut sort = data;
            solution(&mut sort);
            data.sort();
            assert_eq!(sort, data);
        }

        fn test_trend_increasing(solution: fn(&mut [i32])) {
            let mut data: [i32; 1000] = read_bench_data!("trend-increasing");
            let mut sort = data;
            solution(&mut sort);
            data.sort();
            assert_eq!(sort, data);
        }

        fn test_gaussian_with_noise(solution: fn(&mut [i32])) {
            let mut data: [i32; 1000] = read_bench_data!("gaussian-with-noise");
            let mut sort = data;
            solution(&mut sort);
            data.sort();
            assert_eq!(sort, data);
        }

        fn test_low_sample_sin_with_noise(solution: fn(&mut [i32])) {
            let mut data: [i32; 1000] = read_bench_data!("low-sample-sin-with-noise");
            let mut sort = data;
            solution(&mut sort);
            data.sort();
            assert_eq!(sort, data);
        }

        fn test_high_sample_sin_with_noise(solution: fn(&mut [i32])) {
            let mut data: [i32; 1000] = read_bench_data!("high-sample-sin-with-noise");
            let mut sort = data;
            solution(&mut sort);
            data.sort();
            assert_eq!(sort, data);
        }

        pub(super) fn test_all(solution: fn(&mut [i32])) {
            test_empty(solution);
            test_one(solution);
            test_two(solution);
            test_three(solution);
            test_four(solution);
            test_five(solution);
            test_origin(solution);
            test_simple(solution);
            test_random(solution);
            test_stroll(solution);
            test_trend_increasing(solution);
            test_gaussian_with_noise(solution);
            test_low_sample_sin_with_noise(solution);
            test_high_sample_sin_with_noise(solution);
        }
    }

    #[test]
    fn bubble() {
        utils::test_all(bubble_sort);
    }

    #[test]
    fn selection() {
        utils::test_all(selection_sort);
    }

    #[test]
    fn insertion() {
        utils::test_all(insertion_sort);
    }

    #[test]
    fn merge() {
        utils::test_all(merge_sort);
    }

    #[test]
    fn merge_parallel() {
        utils::test_all(merge_sort_parallel);
    }

    #[test]
    fn radix_lsd() {
        utils::test_all(radix_lsd_sort);
    }

    #[test]
    fn qsort_cstd() {
        utils::test_all(cstd_qsort);
    }

    #[test]
    fn quick() {
        utils::test_all(quick_sort);
    }

    mod bench {
        use super::*;
        use crate::read_bench_data;
        extern crate test;

        fn bench_random(b: &mut test::Bencher, solution: fn(&mut [i32])) {
            let mut data: [i32; 10000] = read_bench_data!("random");
            b.iter(|| solution(&mut data));
        }

        fn bench_stroll(b: &mut test::Bencher, solution: fn(&mut [i32])) {
            let mut data: [i32; 10000] = read_bench_data!("stroll");
            b.iter(|| solution(&mut data));
        }

        fn bench_trend_increasing(b: &mut test::Bencher, solution: fn(&mut [i32])) {
            let mut data: [i32; 1000] = read_bench_data!("trend-increasing");
            b.iter(|| solution(&mut data));
        }

        fn bench_gaussian_with_noise(b: &mut test::Bencher, solution: fn(&mut [i32])) {
            let mut data: [i32; 1000] = read_bench_data!("gaussian-with-noise");
            b.iter(|| solution(&mut data));
        }

        fn bench_low_sample_sin_with_noise(b: &mut test::Bencher, solution: fn(&mut [i32])) {
            let mut data: [i32; 1000] = read_bench_data!("low-sample-sin-with-noise");
            b.iter(|| solution(&mut data));
        }

        fn bench_high_sample_sin_with_noise(b: &mut test::Bencher, solution: fn(&mut [i32])) {
            let mut data: [i32; 1000] = read_bench_data!("high-sample-sin-with-noise");
            b.iter(|| solution(&mut data));
        }

        macro_rules! bench_data {
            ($solution:ident, $name:ident) => {
                #[bench]
                fn $name(b: &mut test::Bencher) {
                    concat_idents!(bench_, $name)(b, $solution);
                }
            };
        }

        macro_rules! bench_all {
            ($solution:ident, $($name:ident),*) => {
                $( bench_data!($solution, $name);)*
            };
        }

        mod merge {
            use super::*;

            bench_all!(
                merge_sort,
                random,
                stroll,
                trend_increasing,
                gaussian_with_noise,
                low_sample_sin_with_noise,
                high_sample_sin_with_noise
            );
        }

        mod merge_parallel {
            use super::*;

            bench_all!(
                merge_sort_parallel,
                random,
                stroll,
                trend_increasing,
                gaussian_with_noise,
                low_sample_sin_with_noise,
                high_sample_sin_with_noise
            );
        }

        mod bubble {
            use super::*;

            bench_all!(
                bubble_sort,
                random,
                stroll,
                trend_increasing,
                gaussian_with_noise,
                low_sample_sin_with_noise,
                high_sample_sin_with_noise
            );
        }

        mod selection {
            use super::*;

            bench_all!(
                selection_sort,
                random,
                stroll,
                trend_increasing,
                gaussian_with_noise,
                low_sample_sin_with_noise,
                high_sample_sin_with_noise
            );
        }

        mod insertion {
            use super::*;

            bench_all!(
                insertion_sort,
                random,
                stroll,
                trend_increasing,
                gaussian_with_noise,
                low_sample_sin_with_noise,
                high_sample_sin_with_noise
            );
        }

        mod radix_lsd {
            use super::*;

            bench_all!(
                radix_lsd_sort,
                random,
                stroll,
                trend_increasing,
                gaussian_with_noise,
                low_sample_sin_with_noise,
                high_sample_sin_with_noise
            );
        }

        mod cstd_qsort {
            use super::*;

            bench_all!(
                cstd_qsort,
                random,
                stroll,
                trend_increasing,
                gaussian_with_noise,
                low_sample_sin_with_noise,
                high_sample_sin_with_noise
            );
        }

        mod rust_stable {
            use super::*;

            fn rust_stable(data: &mut [i32]) {
                data.sort();
            }

            bench_all!(
                rust_stable,
                random,
                stroll,
                trend_increasing,
                gaussian_with_noise,
                low_sample_sin_with_noise,
                high_sample_sin_with_noise
            );
        }

        mod quick_sort {
            use super::*;

            bench_all!(
                quick_sort,
                random,
                stroll,
                trend_increasing,
                gaussian_with_noise,
                low_sample_sin_with_noise,
                high_sample_sin_with_noise
            );
        }
    }
}
