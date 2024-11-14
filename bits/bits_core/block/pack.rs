use crate::bits::Bits;
use crate::block::{Block, BlockMut};
use crate::word::Word;

/// Provides helper methods to read/write auxiliary data for Rank and Select.
/// This library should not be used to compress/decompress a large array.
/// Consider using [`quickwit-oss/bitpacking`](https://github.com/quickwit-oss/bitpacking) in such cases.
pub trait Pack: BlockMut {
    fn pack<T: Word>(&mut self, i: usize, n: usize, bits: T) {
        debug_assert!(i < Self::BITS && n <= T::BITS);

        for b in i..i + n {
            // match bits.test(b - i) {
            //     Some(true) => self.set1(b),
            //     Some(false) => self.set0(b),
            //     None => {
            //         break;
            //     }
            // };
            if bits.test(b - i).unwrap_or_default() {
                self.set1(b);
            } else {
                self.set0(b)
            }
        }
    }

    fn unpack<T: Word>(&self, i: usize, n: usize) -> T {
        debug_assert!(i < Self::BITS && n <= T::BITS);

        let mut bits = T::empty();
        for b in i..i + n {
            if self.test(b).unwrap_or_default() {
                bits.set1(b - i);
            }
        }
        bits
    }
}

impl<B: Copy + Block + Pack, const N: usize> Pack for [B; N] {
    fn pack<T: Word>(&mut self, i: usize, n: usize, bits: T) {
        Bits::new_mut(self.as_mut_slice()).pack(i, n, bits)
    }
    fn unpack<T: Word>(&self, i: usize, n: usize) -> T {
        Bits::new(self.as_slice()).unpack(i, n)
    }
}

impl<B: Block + Pack> Pack for Box<B> {
    fn pack<T: Word>(&mut self, i: usize, n: usize, bits: T) {
        self.as_mut().pack(i, n, bits)
    }
    #[inline]
    fn unpack<T: Word>(&self, i: usize, n: usize) -> T {
        self.as_ref().unpack(i, n)
    }
}
