#![allow(missing_docs)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::borrow::Cow;

use bits::block::{
    Block,
    Buf,
    SmallSet,
    *,
};
use bits::{
    BitVec,
    Bits,
};

#[test]
fn block_is_implemented() {
    fn _test<T: Block>() {
        // let bv = BitVec::<T>::new(10);
        // assert_eq!(bv.bits(), T::BITS * 10);
    }
    _test::<[u8; 1]>();

    let bv: Cow<Bits<_>> = Cow::Owned(BitVec::<u64>::new(100));
    assert_eq!(bv.bits(), 128);
}

#[test]
fn bits_is_implemented() {
    fn _test<T>()
    where
        T: Block,
    {
    }

    _test::<[u8; 3]>();
    // _test::<Box<[u8; 3]>>();
    // _test::<[Box<[u8; 3]>; 3]>();
    // _test::<Cow<[u8; 1000]>>();
    // _test::<Cow<Box<[u8; 2000]>>>();
    _test::<Buf<[u8; 100]>>();
    _test::<SmallSet<u16, 10>>();
}

#[quickcheck]
fn rank_count(vec: Vec<u32>) -> bool {
    let vec = BitVec::from(vec);
    vec.count0() == vec.rank0(..) && vec.count1() == vec.rank1(..)
}

#[quickcheck]
fn bits_rank0_rank1(vec: Vec<u32>) -> bool {
    let vec = BitVec::from(vec);
    vec.bits() == vec.rank1(..) + vec.rank0(..)
}

fn rank_for_empty_range<T>(bits: &Bits<T>)
where
    T: Block + Rank,
{
    assert_eq!(bits.rank0(0..0), 0);
    assert_eq!(bits.rank0(1..1), 0);
    assert_eq!(bits.rank0(2..2), 0);
    assert_eq!(bits.rank0(7..7), 0);

    assert_eq!(bits.rank1(0..0), 0);
    assert_eq!(bits.rank1(1..1), 0);
    assert_eq!(bits.rank1(2..2), 0);
    assert_eq!(bits.rank1(7..7), 0);
}

fn rank_0_plus_rank_1<T>(bits: &Bits<T>, r: core::ops::Range<usize>)
where
    T: Block + Rank,
{
    assert_eq!(bits.rank0(r.clone()) + bits.rank1(r.clone()), r.len());
}

#[test]
fn bit_rank() {
    rank_for_empty_range::<u8>(Bits::new(&[!0]));
    rank_for_empty_range::<u8>(Bits::new(&[!0, !0, !0, !0]));

    rank_0_plus_rank_1::<u64>(Bits::new(&[0b_1010_1010]), 0..10);
    rank_0_plus_rank_1::<u64>(Bits::new(&[0b_1010_1010]), 7..20);
    rank_0_plus_rank_1::<[u8; 4]>(Bits::new(&[[!0, 0b_1010_1010, !0, 0b_1010_1010]]), 0..10);
    rank_0_plus_rank_1::<[u8; 4]>(Bits::new(&[[!0, 0b_1010_1010, !0, 0b_1010_1010]]), 7..20);
}

// #[test]
// fn aux() {
//     let mut bv = bitvec::Rho::new((1 << 32) + 10_000);
//     bv.set_bit(1 << 32);
//     assert!(bv.bit(1 << 32).unwrap_or_default());
//     bv.unset_bit(1 << 32);
//     assert!(!bv.bit(1 << 32).unwrap_or_default());
// }

#[quickcheck]
fn repr_bits(vec: Vec<u32>) -> bool {
    let bv = BitVec::from(vec.clone());
    let pop = bits::Pop::from(vec);
    bv.bits() == pop.bits()
}

#[quickcheck]
fn repr_rank(vec: Vec<u32>) -> bool {
    let bv = BitVec::from(vec.clone());
    let aux = bits::Pop::from(vec);
    bv.count1() == aux.count1() && bv.count0() == aux.count0()
}

#[quickcheck]
fn repr_select1(vec: Vec<u32>) -> bool {
    let bv = BitVec::from(vec.clone());
    let aux = bits::Pop::from(vec);
    (0..bv.count1()).all(|i| bv.select1(i) == aux.select1(i))
}

#[quickcheck]
fn repr_select0(vec: Vec<u32>) -> bool {
    let bv = BitVec::from(vec.clone());
    let aux = bits::Pop::from(vec);
    (0..bv.count0()).all(|i| bv.select0(i) == aux.select0(i))
}

fn none<T: Block>(n: usize) -> bits::Pop<T> {
    bits::Pop::new(n)
}

fn setup_bits(size: usize, mut bits: Vec<usize>) -> Vec<usize> {
    bits.push(0);
    bits.push((1 << 16) - 512);
    bits.push(1 << 16);
    bits.push((1 << 16) + 512);
    bits.push(1 << 20);
    bits.push(1 << 32);
    bits.push((1 << 32) + 65530);

    let mut bits = bits.into_iter().filter(|&x| x < size).collect::<Vec<_>>();
    bits.sort();
    bits.dedup();
    bits
}

fn check<T>(size: usize, bits: Vec<usize>) -> bool
where
    T: Block + BlockMut + Rank + Select + Pack,
{
    let mut aux = none::<T>(size);

    for &b in &bits {
        aux.set1(b);
    }

    assert_eq!(aux.count1(), bits.len());

    bits.into_iter().enumerate().all(|(i, b)| {
        aux.test(b).unwrap()
            && aux.rank1(..b) == i
            && aux.select1(i) == Some(b)
            && aux.inner().select1(i) == Some(b)
    })
}

#[quickcheck]
fn bits_u64(bits: Vec<usize>) -> bool {
    let size = 1 << 18;
    let bits = setup_bits(size, bits);

    check::<u64>(size, bits)
}

#[quickcheck]
fn bits_boxed_array(bits: Vec<usize>) -> bool {
    let size = (1 << 32) + 65536;
    let bits = setup_bits(size, bits);

    check::<Box<[u64; 1024]>>(size, bits)
}
