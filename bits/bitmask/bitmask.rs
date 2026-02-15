use std::borrow::Cow;
use std::cmp::Ordering;
use std::iter::Enumerate;
use std::slice;

mod and;
mod not;
mod or;
mod xor;

pub mod helper;

pub use and::*;
pub use not::*;
pub use or::*;
pub use xor::*;

pub trait Mask: Sized {
    type Bits;
    type Iter: Iterator<Item = (usize, Self::Bits)>;

    fn into_mask(self) -> Self::Iter;

    #[inline]
    fn and<That: Mask>(self, that: That) -> And<Self, That> {
        And { a: self, b: that }
    }

    #[inline]
    fn not<That: Mask>(self, that: That) -> Not<Self, That> {
        Not { a: self, b: that }
    }

    #[inline]
    fn or<That: Mask>(self, that: That) -> Or<Self, That> {
        Or { a: self, b: that }
    }

    #[inline]
    fn xor<That: Mask>(self, that: That) -> Xor<Self, That> {
        Xor { a: self, b: that }
    }
}

impl<'inner, T: ?Sized> Mask for &&'inner T
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

impl<'a, T: bitop::Block> Mask for &'a [T] {
    type Bits = Cow<'a, T>;
    type Iter = Blocks<'a, T>;
    fn into_mask(self) -> Self::Iter {
        Blocks { blocks: self.iter().enumerate() }
    }
}

pub struct Blocks<'a, T> {
    blocks: Enumerate<slice::Iter<'a, T>>,
}

impl<'a, T: bitop::Block> Iterator for Blocks<'a, T> {
    type Item = (usize, Cow<'a, T>);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.blocks.find_map(|(i, b)| bitop::Bits::any(b).then(|| (i, Cow::Borrowed(b))))
    }
}
