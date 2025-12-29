use crate::bits::Bits;
use crate::block::Block;

pub trait Count: Block {
    #[inline]
    fn count1(&self) -> usize {
        Self::BITS - self.count0()
    }

    #[inline]
    fn count0(&self) -> usize {
        Self::BITS - self.count1()
    }

    #[inline]
    fn all(&self) -> bool {
        Self::BITS == 0 || self.count0() == 0
    }

    #[inline]
    fn any(&self) -> bool {
        Self::BITS != 0 && self.count1() > 0
    }
}

impl<B: Copy + Block + Count, const N: usize> Count for [B; N] {
    #[inline]
    fn count1(&self) -> usize {
        Bits::new(self.as_slice()).count1()
    }
    #[inline]
    fn count0(&self) -> usize {
        Bits::new(self.as_slice()).count0()
    }
    #[inline]
    fn all(&self) -> bool {
        Bits::new(self.as_slice()).all()
    }
    #[inline]
    fn any(&self) -> bool {
        Bits::new(self.as_slice()).any()
    }
}

impl<B: Block + Count> Count for Box<B> {
    #[inline]
    fn count1(&self) -> usize {
        self.as_ref().count1()
    }
    #[inline]
    fn count0(&self) -> usize {
        self.as_ref().count0()
    }
    #[inline]
    fn all(&self) -> bool {
        self.as_ref().all()
    }
    #[inline]
    fn any(&self) -> bool {
        self.as_ref().any()
    }
}
