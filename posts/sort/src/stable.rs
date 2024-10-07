use crate::{Solution, Sorted};
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Stable;

impl<T: Ord, const N: usize> Solution<T, N> for Stable {
    fn sort_method<F>(array: [T; N], is_less: F) -> Sorted<T, N, impl FnMut(&T, &T) -> bool>
    where
        F: FnMut(&T, &T) -> bool,
    {
        let mut is_less = is_less;
        let mut array = array;

        <[T]>::sort_by(&mut array, |a, b| {
            if is_less(a, b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        unsafe { Sorted::uncheck_from_array(array, is_less) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn simple() {
        let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let a = Stable::sort_method(array, |a, b| a < b);
        assert!(a.force_check());
        assert_eq!(a.take(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn empty() {
        test_empty(Stable);
    }

    #[test]
    fn random() {
        test_random(Stable);
    }

    #[test]
    fn stroll() {
        test_stroll(Stable);
    }

    #[test]
    fn trend_increasing() {
        test_trend_increasing(Stable);
    }

    #[test]
    fn gaussian_with_noise() {
        test_gaussian_with_noise(Stable);
    }

    #[test]
    fn low_sample_sin_with_noise() {
        test_low_sample_sin_with_noise(Stable);
    }

    #[test]
    fn high_sample_sin_with_noise() {
        test_high_sample_sin_with_noise(Stable);
    }
}
