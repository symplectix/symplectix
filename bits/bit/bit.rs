use core::iter::successors;
use core::ops::{Bound, Range, RangeBounds};

// TODO: Use type parameters instead of an argument.
// Type parameters can not be used in const expressions.
// Blocked by Rust issue #60551.
// #[inline]
// pub const fn address<const N: usize>(i: usize) -> (usize, usize) {
//     (i / N, i % N)
// }

#[inline]
pub const fn addr(i: usize, b: usize) -> (usize, usize) {
    (i / b, i % b)
}

/// Calculates the minimum number of blocks to store `n` bits.
#[inline]
pub const fn blocks(n: usize, b: usize) -> usize {
    n / b + (n % b > 0) as usize
}

/// A utility to clamp the given range, which is possibly unbounded, into a bounded one.
/// Panics when debug is enabled and if `!(min <= i && i <= j && j <= max)`.
pub fn bounded<R>(r: &R, min: usize, max: usize) -> Range<usize>
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

/// Splits a given range [s, e) into chunks.
/// Each chunk is represented as a (index, len) tuple, and its rhs, index+len, is aligned to a multiple of n.
///
/// # Examples
///
/// ```
/// let mut it = bit::chunks(10, 0, 3);
/// assert_eq!(it.next(), None);
///
/// let mut it = bit::chunks(10, 10, 3);
/// assert_eq!(it.next(), None);
///
/// let mut it = bit::chunks(10, 12, 3);
/// assert_eq!(it.next(), Some((10, 2)));
/// assert_eq!(it.next(), None);
///
/// let mut it = bit::chunks(10, 20, 3);
/// assert_eq!(it.next(), Some((10, 2)));
/// assert_eq!(it.next(), Some((12, 3)));
/// assert_eq!(it.next(), Some((15, 3)));
/// assert_eq!(it.next(), Some((18, 2)));
/// assert_eq!(it.next(), None);
///
/// let mut it = bit::chunks(10, 21, 3);
/// assert_eq!(it.next(), Some((10, 2)));
/// assert_eq!(it.next(), Some((12, 3)));
/// assert_eq!(it.next(), Some((15, 3)));
/// assert_eq!(it.next(), Some((18, 3)));
/// assert_eq!(it.next(), None);
/// ```
pub fn chunks(start: usize, end: usize, n: usize) -> impl Iterator<Item = (usize, usize)> {
    let step = move |i| (i < end).then(|| (i, next_multiple_of(i, n).min(end) - i));
    successors(step(start), move |&(index, len)| step(index + len))
}

// TODO: Use [usize::checked_next_multiple_of](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_next_multiple_of).
// https://github.com/rust-lang/rust/issues/88581
#[inline]
const fn next_multiple_of(x: usize, n: usize) -> usize {
    x + (n - x % n)
}

#[cfg(test)]
mod tests {
    #[test]
    fn next_multiple_of() {
        use super::next_multiple_of;
        assert_eq!(next_multiple_of(0, 8), 8);
        assert_eq!(next_multiple_of(12, 3), 15);
        assert_eq!(next_multiple_of(16, 8), 24);
        assert_eq!(next_multiple_of(23, 8), 24);
        assert_eq!(next_multiple_of(9, 3), 12);
    }
}
