use core::ops::{Bound, Range, RangeBounds};

/// A utility to clamp the given range into a valid one.
/// Panics if debug is enabled and `min <= i && i <= j && j <= max`.
pub(crate) fn to_range<R>(r: &R, min: usize, max: usize) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    let i = min_index_inclusive(r.start_bound(), min);
    let j = max_index_exclusive(r.end_bound(), max);
    debug_assert!(min <= i && i <= j && j <= max);
    i..j
}

#[inline]
const fn min_index_inclusive(bound: Bound<&usize>, min: usize) -> usize {
    match bound {
        Bound::Included(&s) => s,
        Bound::Excluded(&s) => s + 1,
        Bound::Unbounded => min,
    }
}

#[inline]
const fn max_index_exclusive(bound: Bound<&usize>, max: usize) -> usize {
    match bound {
        Bound::Included(&e) => e + 1,
        Bound::Excluded(&e) => e,
        Bound::Unbounded => max,
    }
}

// pub fn to_range_inclusive<R>(r: &R, min: usize, max: usize) -> RangeInclusive<usize>
// where
//     R: RangeBounds<usize>,
// {
//     let i = min_index_inclusive(r.start_bound(), min);
//     let j = max_index_inclusive(r.end_bound(), max);
//     debug_assert!(min <= i && i <= j && j <= max);
//     i..=j
// }

// #[inline]
// pub fn min_index_exclusive(bound: Bound<&usize>, min: usize) -> usize {
//     match bound {
//         Bound::Included(&s) => s + 1,
//         Bound::Excluded(&s) => s,
//         Bound::Unbounded => min,
//     }
// }

// #[inline]
// pub fn max_index_inclusive(bound: Bound<&usize>, max: usize) -> usize {
//     match bound {
//         Bound::Included(&e) => e,
//         Bound::Excluded(&e) => e - 1,
//         Bound::Unbounded => max,
//     }
// }
