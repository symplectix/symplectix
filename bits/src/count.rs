use crate::{Bits, Block};

pub trait Count: Bits {
    /// Counts the occurrences of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Count;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.count1(), 0);
    /// assert_eq!(b.count1(), 0);
    /// assert_eq!(c.count1(), 3);
    /// ```
    #[inline]
    fn count1(&self) -> usize {
        self.bits() - self.count0()
    }

    /// Counts the occurrences of `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Count;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.count0(), 0);
    /// assert_eq!(b.count0(), 192);
    /// assert_eq!(c.count0(), 189);
    /// ```
    #[inline]
    fn count0(&self) -> usize {
        self.bits() - self.count1()
    }

    /// Returns true if all bits are enabled. An empty bits should return true.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Count;
    /// let a: &[u64] = &[0, 0, 0];
    /// let b: &[u64] = &[];
    /// let c: &[u64] = &[!0, !0, !0];
    /// assert!(!a.all());
    /// assert!( b.all());
    /// assert!( c.all());
    /// ```
    #[inline]
    fn all(&self) -> bool {
        self.bits() == 0 || self.count0() == 0
    }

    /// Returns true if any bits are enabled. An empty bits should return false.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Count;
    /// let b1: &[u64] = &[];
    /// let b2: &[u64] = &[0, 0, 0];
    /// let b3: &[u64] = &[!0, !0, !0];
    /// let b4: &[u64] = &[0, 0, 1];
    /// assert!(!b1.any());
    /// assert!(!b2.any());
    /// assert!( b3.any());
    /// assert!( b4.any());
    /// ```
    #[inline]
    fn any(&self) -> bool {
        self.bits() != 0 && self.count1() > 0
    }
}

macro_rules! impls {
    ($( $Int:ty )*) => ($(
        impl Count for $Int {
            #[inline]
            fn count1(&self) -> usize {
                self.count_ones() as usize
            }

            #[inline]
            fn count0(&self) -> usize {
                self.count_zeros() as usize
            }

            #[inline]
            fn all(&self) -> bool {
                *self == !0
            }

            #[inline]
            fn any(&self) -> bool {
                *self != 0
            }
        }
    )*)
}
impls!(u8 u16 u32 u64 u128 usize);
impls!(i8 i16 i32 i64 i128 isize);

impl<B: Block> Count for [B] {
    #[inline]
    fn count1(&self) -> usize {
        self.iter().map(Count::count1).sum()
    }

    #[inline]
    fn count0(&self) -> usize {
        self.iter().map(Count::count0).sum()
    }

    #[inline]
    fn all(&self) -> bool {
        self.iter().all(Count::all)
    }

    #[inline]
    fn any(&self) -> bool {
        self.iter().any(Count::any)
    }
}

macro_rules! impl_count {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn count1(&self) -> usize {
            <$X as Count>::count1(self$(.$method())?)
        }

        #[inline]
        fn count0(&self) -> usize {
            <$X as Count>::count0(self$(.$method())?)
        }

        #[inline]
        fn all(&self) -> bool {
            <$X as Count>::all(self$(.$method())?)
        }
        #[inline]
        fn any(&self) -> bool {
            <$X as Count>::any(self$(.$method())?)
        }
    }
}

impl<'a, T: ?Sized + Count> Count for &'a T {
    impl_count!(T);
}

impl<T, const N: usize> Count for [T; N]
where
    [T]: Count,
{
    impl_count!([T], as_ref);
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<B> Count for Vec<B>
    where
        [B]: Count,
    {
        impl_count!([B]);
    }

    impl<T: ?Sized + Count> Count for Box<T> {
        impl_count!(T);
    }

    impl<'a, T> Count for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Count,
    {
        impl_count!(T, as_ref);
    }
}
