#![no_std]

//! `bits`

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod and;
pub mod bits;
pub mod bits_mut;
pub mod block;
pub mod count;
pub mod excess;
pub mod not;
pub mod or;
pub mod rank;
pub mod select;
pub mod word;
pub mod xor;

pub use self::bits::Bits;
pub use self::bits_mut::BitsMut;
pub use self::block::Block;
pub use self::count::Count;
pub use self::excess::Excess;
pub use self::rank::Rank;
pub use self::select::Select;
pub use self::word::Word;
pub use self::{and::And, not::Not, or::Or, xor::Xor};

use core::{
    cmp::Ordering,
    ops::{Bound, Div, Range, RangeBounds, Rem},
};

#[inline]
fn address<T: Block>(i: usize) -> (usize, usize) {
    fn divrem<T, U>(t: T, u: U) -> (<T as Div<U>>::Output, <T as Rem<U>>::Output)
    where
        T: Copy + Div<U> + Rem<U>,
        U: Copy,
    {
        (t / u, t % u)
    }

    divrem(i, T::BITS)
}

/// A utility to clamp the given range into a valid one.
/// Panics if debug is enabled and `min <= i && i <= j && j <= max`.
fn to_range<R: RangeBounds<usize>>(r: &R, min: usize, max: usize) -> (usize, usize) {
    let (i, j) = (
        match r.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => min,
        },
        match r.end_bound() {
            Bound::Included(&e) => e + 1,
            Bound::Excluded(&e) => e,
            Bound::Unbounded => max,
        },
    );

    debug_assert!(min <= i && i <= j && j <= max);
    (i, j)
}

fn for_each_blocks<T, F>(s: usize, e: usize, mut f: F)
where
    T: Block,
    F: FnMut(usize, Range<usize>),
{
    assert!(s <= e);
    if s == e {
        return;
    }

    let (q0, r0) = crate::address::<T>(s);
    let (q1, r1) = crate::address::<T>(e);

    if q0 == q1 {
        f(q0, r0..r1);
    } else {
        f(q0, r0..T::BITS);
        (q0 + 1..q1).for_each(|k| f(k, 0..T::BITS));
        f(q1, 0..r1)
    }
}

fn compare_index<T, U>(
    x: Option<&(usize, T)>,
    y: Option<&(usize, U)>,
    when_x_is_none: Ordering,
    when_y_is_none: Ordering,
) -> Ordering {
    match (x, y) {
        (None, _) => when_x_is_none,
        (_, None) => when_y_is_none,
        (Some((i, _)), Some((j, _))) => i.cmp(j),
    }
}

// /// `Fold` is an iterator built from `Mask`s.
// pub struct Fold<'a, B>(Box<dyn Iterator<Item = (usize, B)> + 'a>);

// impl<'a, T: 'a> Fold<'a, T> {
//     pub(crate) fn fold<A, B, F>(xs: impl IntoIterator<Item = A>, mut f: F) -> Fold<'a, T>
//     where
//         A: 'a + Mask<Block = T>,
//         B: 'a + Mask<Block = T>,
//         F: FnMut(Box<dyn Iterator<Item = (usize, T)> + 'a>, A) -> B,
//     {
//         let mut xs = xs.into_iter();
//         if let Some(acc) = xs.next() {
//             Fold(xs.fold(Box::new(acc.steps()), |a, x| Box::new(f(a, x).steps())))
//         } else {
//             Fold(Box::new(std::iter::empty()))
//         }
//     }

//     /// Folds `xs` into a single iterator that applies `and` to each bits.
//     pub fn and<A>(xs: impl IntoIterator<Item = A>) -> Self
//     where
//         A: 'a + Mask<Block = T>,
//         A::Block: Intersection<A::Block>,
//     {
//         Self::fold(xs, And::new)
//     }

//     /// Folds `xs` into a single iterator that applies `and_not` to each bits.
//     pub fn not<A>(xs: impl IntoIterator<Item = A>) -> Self
//     where
//         A: 'a + Mask<Block = T>,
//         A::Block: Difference<A::Block>,
//     {
//         Self::fold(xs, Not::new)
//     }

//     /// Folds `xs` into a single iterator that applies `or` to each bits.
//     pub fn or<A>(xs: impl IntoIterator<Item = A>) -> Self
//     where
//         A: 'a + Mask<Block = T>,
//         A::Block: Union<A::Block>,
//     {
//         Self::fold(xs, Or::new)
//     }

//     /// Folds `xs` into a single iterator that applies `xor` to each bits.
//     pub fn xor<A>(xs: impl IntoIterator<Item = A>) -> Self
//     where
//         A: 'a + Mask<Block = T>,
//         A::Block: SymmetricDifference<A::Block>,
//     {
//         Self::fold(xs, Xor::new)
//     }
// }

// impl<'a, T> Mask for Box<dyn Iterator<Item = (usize, T)> + 'a> {
//     type Block = T;
//     type Steps = Self;
//     fn steps(self) -> Self::Steps {
//         self.into_iter()
//     }
// }

// impl<'a, T> Mask for Fold<'a, T> {
//     type Block = T;
//     type Steps = Self;
//     #[inline]
//     fn steps(self) -> Self::Steps {
//         self
//     }
// }

// impl<'a, T> Iterator for Fold<'a, T> {
//     type Item = (usize, T);
//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.next()
//     }
//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         self.0.size_hint()
//     }
// }
