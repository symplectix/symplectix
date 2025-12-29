use std::borrow::{
    Borrow,
    BorrowMut,
};
use std::ops;

use crate::{
    Bits,
    Block,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitVec<T> {
    pub(crate) data: Vec<T>,
}

impl<T> ops::Deref for BitVec<T> {
    type Target = Bits<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        Bits::from_slice(self.data.as_slice())
    }
}
impl<T> ops::DerefMut for BitVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        Bits::from_slice_mut(self.data.as_mut_slice())
    }
}

impl<T> Borrow<Bits<T>> for BitVec<T> {
    #[inline]
    fn borrow(&self) -> &Bits<T> {
        self
    }
}
impl<T> BorrowMut<Bits<T>> for BitVec<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Bits<T> {
        self
    }
}

impl<T> From<Vec<T>> for BitVec<T> {
    #[inline]
    fn from(data: Vec<T>) -> Self {
        BitVec { data }
    }
}

impl<T> From<Box<Bits<T>>> for BitVec<T> {
    #[inline]
    fn from(b: Box<Bits<T>>) -> Self {
        b.into_vec()
    }
}

impl<T: Block> BitVec<T> {
    /// Constructs a new, empty `Vec<T>`.
    ///
    /// # Tests
    ///
    /// ```
    /// # use bits_core::BitVec;
    /// let v = BitVec::<u8>::new(30);
    ///
    /// assert_eq!(v.as_slice().len(), 4);
    /// assert_eq!(v.bits(), 32);
    /// assert_eq!(v.count1(), 0);
    /// assert_eq!(v.count0(), 32);
    /// ```
    pub fn new(n: usize) -> BitVec<T> {
        BitVec { data: crate::make(n) }
    }
}
