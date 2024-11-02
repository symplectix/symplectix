use crate::Block;
use core::cmp::Ordering;
use core::marker;
use core::ops::{Bound, Range, RangeBounds};

#[inline]
pub(crate) fn address<T: Block>(i: usize) -> (usize, usize) {
    num::divrem(i, T::BITS)
}

pub(crate) fn between<B: Block>(s: usize, e: usize) -> impl Iterator<Item = (usize, Range<usize>)> {
    struct Between<B> {
        pos: (usize, usize),
        end: (usize, usize),
        _block: marker::PhantomData<B>,
    }

    impl<B: Block> Iterator for Between<B> {
        type Item = (usize, Range<usize>);

        fn next(&mut self) -> Option<Self::Item> {
            let (i, p) = self.pos; // p is 0 except the first item
            let (j, q) = self.end; // q is B::BITS except the last item

            match i.cmp(&j) {
                Ordering::Less => {
                    self.pos = (i + 1, 0);
                    Some((i, p..B::BITS))
                }
                Ordering::Equal => {
                    self.pos = (i + 1, 0);
                    Some((i, p..q))
                }
                Ordering::Greater => None,
            }
        }
    }

    Between { pos: address::<B>(s), end: address::<B>(e), _block: marker::PhantomData::<B> }
}

/// A utility to clamp the given range into a valid one.
/// Panics if debug is enabled and `min <= i && i <= j && j <= max`.
pub(crate) fn to_range<R>(r: &R, min: usize, max: usize) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    let i = min_index_inclusive(r.start_bound(), min);
    let j = max_index_exclusive(r.end_bound(), max);
    assert!(min <= i && i <= j && j <= max);
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

pub(crate) fn compare<X, Y>(
    x: Option<&(usize, X)>,
    y: Option<&(usize, Y)>,
    when_x_is_none: Ordering,
    when_y_is_none: Ordering,
) -> Ordering {
    match (x, y) {
        (None, _) => when_x_is_none,
        (_, None) => when_y_is_none,
        (Some((i, _x)), Some((j, _y))) => i.cmp(j),
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
