mod bit_vec;
mod bits;
pub mod block;
pub mod mask;
pub mod word;

pub use bit_vec::BitVec;
pub use bits::Bits;
use block::Block;
use word::Word;

pub(crate) fn make<T: Block>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty())).take(bit::blocks(n, T::BITS)).collect()
}
