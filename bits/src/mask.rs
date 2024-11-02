use core::cmp::Ordering;

mod and;
mod not;
mod or;
mod xor;

pub use self::{
    and::{And, AndAssign},
    not::{Not, NotAssign},
    or::{Or, OrAssign},
    xor::{Xor, XorAssign},
};

pub use self::{and::Intersection, not::Difference, or::Union, xor::SymmetricDifference};

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
