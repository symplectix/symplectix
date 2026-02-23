use std::ops::{
    Deref,
    DerefMut,
};

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Bits(pub(crate) [u8]);

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BitVec(pub(crate) Vec<u8>);

impl Bits {
    pub(crate) fn new<B: ?Sized + AsRef<[u8]>>(bytes: &B) -> &Bits {
        Bits::from_bytes(bytes.as_ref())
    }

    pub(crate) const fn from_bytes(slice: &[u8]) -> &Bits {
        unsafe { &*(slice as *const [u8] as *const Bits) }
    }

    #[inline]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub(crate) fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    pub(crate) fn num_chunks(&self) -> u32 {
        self.0.is_empty().then_some(0).unwrap_or_else(|| {
            let bs = self.as_bytes();
            // MAX_CHUNKS is 1<<16. -1 to store the chunk value as u16.
            u16::from_le_bytes([bs[0], bs[1]]) as u32 + 1
        })
    }
}

impl Deref for Bits {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bits {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for BitVec {
    type Target = Bits;

    #[inline]
    fn deref(&self) -> &Self::Target {
        Bits::from_bytes(self.0.as_slice())
    }
}

impl BitVec {
    fn from_sorted_bits(nums: impl Iterator<Item = u32>) -> BitVec {
        todo!()
    }
}

// CHUNK_BITS is `u16::MAX + 1`.
// Need `-1` to save cardinalities as [u16].
const CHUNK_BITS: u64 = 1 << 16;
// BLOCK_BITS is `u8::MAX + 1`.
// Need `-1` to save cardinalities as [u8].
const BLOCK_BITS: u64 = 1 << 8;

// Where universe `u` <= `1<<32`.
const MAX_CHUNKS: usize = (1 << 32) / CHUNK_BITS as usize;

const DENSE_CHUNK_BYTES: usize = 1024;
const DENSE_BLOCK_BYTES: usize = 32;

// The block is sparse if cardinality < 31,
// so sparse blocks contain at most 30 integers.
const SPARSE_BLOCK_THRESHOLD: usize = 31;

// HEADER1 contains metadata for each non-empty chunk
// - the index
// - the cardinality
// - the number of bytes of the chunk
// - the number of blocks, equal to the number of header2.
// Each of these values should fit into u16.
const HEADER1_BYTES: usize = 8;

// HEADER2 contains metadata for each non-empty block
// - the index
// - the cardinality
// Each of these values should fit into u8.
const HEADER2_BYTES: usize = 2;
