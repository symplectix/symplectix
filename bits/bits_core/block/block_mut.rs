use super::Block;
use crate::bits::Bits;

pub trait BlockMut: Block {
    /// Enables the bit at the given index `i`.
    fn set1(&mut self, i: usize);

    /// Disables the bit at the given index `i`.
    fn set0(&mut self, i: usize);
}

impl<B: Copy + BlockMut, const N: usize> BlockMut for [B; N] {
    #[inline]
    fn set1(&mut self, i: usize) {
        Bits::new_mut(self.as_mut_slice()).set1(i)
    }
    #[inline]
    fn set0(&mut self, i: usize) {
        Bits::new_mut(self.as_mut_slice()).set0(i)
    }
}

impl<B: BlockMut> BlockMut for Box<B> {
    #[inline]
    fn set1(&mut self, i: usize) {
        self.as_mut().set1(i)
    }
    #[inline]
    fn set0(&mut self, i: usize) {
        self.as_mut().set0(i)
    }
}
