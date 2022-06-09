use core::cmp::Ordering;

pub mod and;
pub mod not;
pub mod or;
pub mod xor;

use self::{and::BitwiseAnd, not::BitwiseNot, or::BitwiseOr, xor::BitwiseXor};

pub trait Mask: Sized {
    type Bits;

    type Iter: Iterator<Item = (usize, Self::Bits)>;

    fn into_mask(self) -> Self::Iter;

    #[inline]
    fn and<That: Mask>(self, that: That) -> BitwiseAnd<Self, That> {
        BitwiseAnd { a: self, b: that }
    }

    #[inline]
    fn not<That: Mask>(self, that: That) -> BitwiseNot<Self, That> {
        BitwiseNot { a: self, b: that }
    }

    #[inline]
    fn or<That: Mask>(self, that: That) -> BitwiseOr<Self, That> {
        BitwiseOr { a: self, b: that }
    }

    #[inline]
    fn xor<That: Mask>(self, that: That) -> BitwiseXor<Self, That> {
        BitwiseXor { a: self, b: that }
    }
}

impl<'inner, 'outer, T: ?Sized> Mask for &'outer &'inner T
where
    &'inner T: Mask,
{
    type Bits = <&'inner T as Mask>::Bits;
    type Iter = <&'inner T as Mask>::Iter;
    #[inline]
    fn into_mask(self) -> Self::Iter {
        Mask::into_mask(*self)
    }
}

impl<'a, B, const N: usize> Mask for &'a [B; N]
where
    &'a [B]: Mask,
{
    type Bits = <&'a [B] as Mask>::Bits;
    type Iter = <&'a [B] as Mask>::Iter;
    #[inline]
    fn into_mask(self) -> Self::Iter {
        self.as_ref().into_mask()
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

mod impl_mask {
    use super::Mask;
    use crate::Bits;
    use std::borrow::Cow;
    use std::{iter::Enumerate, slice};

    impl<'a, T: Bits> Mask for &'a [T] {
        type Bits = Cow<'a, T>;
        type Iter = Blocks<'a, T>;
        fn into_mask(self) -> Self::Iter {
            Blocks { blocks: self.iter().enumerate() }
        }
    }

    pub struct Blocks<'a, T> {
        blocks: Enumerate<slice::Iter<'a, T>>,
    }

    impl<'a, T: Bits> Iterator for Blocks<'a, T> {
        type Item = (usize, Cow<'a, T>);
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.blocks.find_map(|(i, b)| b.any().then(|| (i, Cow::Borrowed(b))))
        }
    }
}
