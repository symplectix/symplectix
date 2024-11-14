use std::cmp::Ordering::{self, Equal as EQ, Greater as GT, Less as LT};
use std::iter::Peekable;
use std::ops::{Deref, DerefMut, Range, RangeBounds};

use bits_core::block::*;
use bits_core::mask::helper;
use smallvec::SmallVec;

#[derive(Debug, Default, Clone)]
pub struct SmallSet<T, const N: usize>(SmallVec<T, N>);

impl<T, const N: usize> AsRef<[T]> for SmallSet<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> AsMut<[T]> for SmallSet<T, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> SmallSet<T, N> {
    #[inline]
    fn as_slice(&self) -> &[T] {
        self.0.deref()
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.deref_mut()
    }
}

impl<const N: usize> SmallSet<u16, N> {
    const fn bits(&self) -> usize {
        Self::BITS
    }
}

impl<const N: usize> Block for SmallSet<u16, N> {
    const BITS: usize = u16::MAX as usize + 1;

    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use smallset::SmallSet;
    /// assert_eq!(SmallSet::<u16, 4>::BITS, 65536);
    /// ```
    #[inline]
    fn empty() -> Self {
        SmallSet(SmallVec::new())
    }

    #[inline]
    fn test(&self, i: usize) -> Option<bool> {
        num::cast(i).and_then(|k| self.as_slice().binary_search(&k).map(|_found| true).ok())
    }
}

impl<const N: usize> BlockMut for SmallSet<u16, N> {
    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// let mut b = smallset::SmallSet::<u16, 4>::empty();
    ///
    /// b.set1(100);
    /// assert_eq!(b.test(100), Some(true));
    /// assert_eq!(b.count1(), 1);
    /// ```
    #[inline]
    fn set1(&mut self, i: usize) {
        assert!(i < self.bits());
        let i = num::cast(i).unwrap();
        if let Err(k) = self.as_slice().binary_search(&i) {
            self.0.insert(k, i);
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// let mut b = smallset::SmallSet::<u16, 4>::empty();
    ///
    /// b.set1(100);
    /// assert_eq!(b.test(100), Some(true));
    /// b.set0(100);
    /// assert_eq!(b.test(100), None);
    /// ```
    #[inline]
    fn set0(&mut self, i: usize) {
        assert!(i < self.bits());
        let i = num::cast(i).unwrap();
        if let Ok(k) = self.as_slice().binary_search(&i) {
            self.0.remove(k);
        }
    }
}

impl<const N: usize> Count for SmallSet<u16, N> {
    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// let mut b = smallset::SmallSet::<u16, 12>::empty();
    ///
    /// b.set1(300);
    /// b.set1(200);
    /// b.set1(100);
    /// assert_eq!(b.count1(), 3);
    /// ```
    #[inline]
    fn count1(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Rank for SmallSet<u16, N> {
    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// let mut b = smallset::SmallSet::<u16, 12>::empty();
    ///
    /// b.set1(65530);
    /// b.set1(65520);
    /// b.set1(65510);
    /// assert_eq!(b.rank1(..), 3);
    /// assert_eq!(b.rank1(..65530), 2);
    /// assert_eq!(b.rank1(..65536), 3);
    /// ```
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let rank = |i| {
            let i = num::cast(i).unwrap();
            // Search the smallest index `p` that satisfy `vec[p] >= i`,
            // `p` also implies the number of enabled bits in [0, p).
            // For example, searching 5 in `[0, 1, 7]` return 2.
            match self.as_slice().binary_search(&i) {
                Ok(p) | Err(p) => p,
            }
        };

        let cap = self.bits();
        let Range { start: i, end: j } = bit::bounded(&r, 0, cap);
        match (i, j) {
            (i, j) if i == j => 0,
            (0, n) if n == cap => self.count1(),
            (0, n) => rank(n),
            (i, j) if j == cap => self.count1() - rank(i),
            (i, j) => rank(j) - rank(i),
        }
    }
}

impl<const N: usize> Select for SmallSet<u16, N> {
    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// let mut b = smallset::SmallSet::<u16, 12>::empty();
    ///
    /// b.set1(65530);
    /// b.set1(65520);
    /// b.set1(65510);
    /// assert_eq!(b.select1(0), Some(65510));
    /// assert_eq!(b.select1(1), Some(65520));
    /// assert_eq!(b.select1(2), Some(65530));
    /// assert_eq!(b.select1(3), None);
    /// ```
    fn select1(&self, n: usize) -> Option<usize> {
        self.as_slice().get(n).map(|&u| u as usize)
    }
}

fn cmp_opt<T: Ord>(a: Option<&T>, b: Option<&T>, a_is_none: Ordering, b_is_none: Ordering) -> Ordering {
    match (a, b) {
        (None, _) => a_is_none,
        (_, None) => b_is_none,
        (Some(x), Some(y)) => x.cmp(y),
    }
}

impl<const N: usize, const M: usize> helper::Assign<SmallSet<u16, M>> for SmallSet<u16, N> {
    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use bits_core::mask::helper::Assign;
    /// let mut a = smallset::SmallSet::<u16, 4>::empty();
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(3);
    ///
    /// let mut b = smallset::SmallSet::<u16, 4>::empty();
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(4);
    ///
    /// Assign::and(&mut a, &b);
    /// assert_eq!(a.as_ref(), &[2, 3]);
    /// ```
    fn and(a: &mut Self, b: &SmallSet<u16, M>) {
        a.0 = Iter { a: a.as_slice().iter().peekable(), b: b.as_slice().iter().peekable() }.collect();

        struct Iter<A: Iterator, B: Iterator> {
            a: Peekable<A>,
            b: Peekable<B>,
        }
        impl<'a, 'b, A, B> Iterator for Iter<A, B>
        where
            A: Iterator<Item = &'a u16>,
            B: Iterator<Item = &'b u16>,
        {
            type Item = u16;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match self.a.peek()?.cmp(self.b.peek()?) {
                        LT => {
                            self.a.next();
                        }
                        EQ => {
                            let a = self.a.next().unwrap();
                            let b = self.b.next().unwrap();
                            debug_assert_eq!(a, b);
                            break Some(*a);
                        }
                        GT => {
                            self.b.next();
                        }
                    }
                }
            }
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use bits_core::mask::helper::Assign;
    /// let mut a = smallset::SmallSet::<u16, 4>::empty();
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(3);
    ///
    /// let mut b = smallset::SmallSet::<u16, 4>::empty();
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(4);
    ///
    /// Assign::not(&mut a, &b);
    /// assert_eq!(a.as_ref(), &[1]);
    /// ```
    fn not(a: &mut Self, b: &SmallSet<u16, M>) {
        a.0 = Iter { a: a.as_slice().iter().peekable(), b: b.as_slice().iter().peekable() }.collect();

        struct Iter<A: Iterator, B: Iterator> {
            a: Peekable<A>,
            b: Peekable<B>,
        }
        impl<'a, 'b, A, B> Iterator for Iter<A, B>
        where
            A: Iterator<Item = &'a u16>,
            B: Iterator<Item = &'b u16>,
        {
            type Item = u16;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match cmp_opt(self.a.peek(), self.b.peek(), LT, LT) {
                        LT => break self.a.next().copied(),
                        EQ => {
                            let a = self.a.next().unwrap();
                            let b = self.b.next().unwrap();
                            debug_assert_eq!(a, b);
                        }
                        GT => {
                            self.b.next().unwrap();
                        }
                    }
                }
            }
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use bits_core::mask::helper::Assign;
    /// let mut a = smallset::SmallSet::<u16, 4>::empty();
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(3);
    ///
    /// let mut b = smallset::SmallSet::<u16, 4>::empty();
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(4);
    ///
    /// Assign::or(&mut a, &b);
    /// assert_eq!(a.as_ref(), &[1, 2, 3, 4]);
    /// ```
    fn or(a: &mut Self, b: &SmallSet<u16, M>) {
        a.0 = Iter { a: a.as_slice().iter().peekable(), b: b.as_slice().iter().peekable() }.collect();

        struct Iter<A: Iterator, B: Iterator> {
            a: Peekable<A>,
            b: Peekable<B>,
        }
        impl<'a, 'b, A, B> Iterator for Iter<A, B>
        where
            A: Iterator<Item = &'a u16>,
            B: Iterator<Item = &'b u16>,
        {
            type Item = u16;

            fn next(&mut self) -> Option<Self::Item> {
                match cmp_opt(self.a.peek(), self.b.peek(), GT, LT) {
                    LT => self.a.next().copied(),
                    EQ => {
                        let a = self.a.next().unwrap();
                        let b = self.b.next().unwrap();
                        debug_assert_eq!(a, b);
                        Some(*a)
                    }
                    GT => self.b.next().copied(),
                }
            }
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use bits_core::mask::helper::Assign;
    /// let mut a = smallset::SmallSet::<u16, 4>::empty();
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(3);
    ///
    /// let mut b = smallset::SmallSet::<u16, 4>::empty();
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(4);
    ///
    /// Assign::xor(&mut a, &b);
    /// assert_eq!(a.as_ref(), &[1, 4]);
    /// ```
    fn xor(a: &mut Self, b: &SmallSet<u16, M>) {
        a.0 = Iter { a: a.as_slice().iter().peekable(), b: b.as_slice().iter().peekable() }.collect();

        struct Iter<L: Iterator, R: Iterator> {
            a: Peekable<L>,
            b: Peekable<R>,
        }
        impl<'a, 'b, A, B> Iterator for Iter<A, B>
        where
            A: Iterator<Item = &'a u16>,
            B: Iterator<Item = &'b u16>,
        {
            type Item = u16;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match cmp_opt(self.a.peek(), self.b.peek(), GT, LT) {
                        LT => break self.a.next().copied(),
                        EQ => {
                            let a = self.a.next().unwrap();
                            let b = self.b.next().unwrap();
                            debug_assert_eq!(a, b);
                        }
                        GT => break self.b.next().copied(),
                    }
                }
            }
        }
    }
}
