use core::cmp::Ordering;
use core::ops::{Bound, Range, RangeBounds};

// TODO: Use type parameters instead of an argument.
// Type parameters can not be used in const expressions.
// Blocked by Rust issue #60551.
// #[inline]
// pub const fn address<const N: usize>(i: usize) -> (usize, usize) {
//     (i / N, i % N)
// }

#[inline]
pub const fn address(i: usize, b: usize) -> (usize, usize) {
    (i / b, i % b)
}

/// A utility to clamp the given range, which is possibly unbounded, into a bounded one.
/// Panics when debug is enabled and if `min <= i && i <= j && j <= max`.
pub fn bounded<R>(r: &R, min: usize, max: usize) -> Range<usize>
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

pub const fn between(
    start: usize,
    end: usize,
    step: usize,
) -> impl Iterator<Item = (usize, Range<usize>)> {
    struct Between {
        current: (usize, usize),
        end: (usize, usize),
        sep: usize,
    }

    impl Iterator for Between {
        type Item = (usize, Range<usize>);

        fn next(&mut self) -> Option<Self::Item> {
            let (i, p) = self.current; // p is 0 except the first item
            let (j, q) = self.end; // q is B::BITS except the last item
            let sep = self.sep;

            match i.cmp(&j) {
                Ordering::Less => {
                    self.current = (i + 1, 0);
                    Some((i, p..sep))
                }
                Ordering::Equal => {
                    self.current = (i + 1, 0);
                    Some((i, p..q))
                }
                Ordering::Greater => None,
            }
        }
    }

    Between { current: address(start, step), end: address(end, step), sep: step }
}

#[cfg(test)]
mod tests {
    #[test]
    fn between() {
        let mut it = super::between(10, 20, 3);
        assert_eq!(it.next(), Some((3, 1..3))); // 10..12
        assert_eq!(it.next(), Some((4, 0..3))); // 12..15
        assert_eq!(it.next(), Some((5, 0..3))); // 15..18
        assert_eq!(it.next(), Some((6, 0..2))); // 18..20
        assert_eq!(it.next(), None);
    }
}
