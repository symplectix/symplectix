#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::borrow::Cow;

use bits::block::{Block, BoxContainer};
use bits::{Bits, BitsMut};
use bits_pack::Unpack;

#[test]
fn bits_is_implemented() {
    fn _test<T>()
    where
        T: ?Sized + bits::Bits,
    {
    }

    _test::<&u8>();
    _test::<[u8; 1]>();
    _test::<&[u8; 1]>();
    _test::<[u8]>();
    _test::<&[u8]>();
    _test::<Vec<[u8; 1]>>();
    _test::<&Vec<[u8; 2]>>();
    _test::<Box<[u8; 3]>>();
    _test::<[Box<[u8; 3]>]>();
    _test::<&Box<[u8; 4]>>();
    _test::<Cow<[u8; 1000]>>();
    _test::<Cow<Box<[u8; 2000]>>>();
    _test::<BoxContainer<[u8; 100]>>();
}

#[quickcheck]
fn rank_count(vec: Vec<u32>) -> bool {
    vec.count0() == vec.rank0(..) && vec.count1() == vec.rank1(..)
}

#[quickcheck]
fn bits_rank0_rank1(vec: Vec<u32>) -> bool {
    Bits::bits(&vec) == vec.rank1(..) + vec.rank0(..)
}

fn rank_for_empty_range<T>(bits: &T)
where
    T: ?Sized + Bits,
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

fn rank_0_plus_rank_1<T>(bits: &T, r: core::ops::Range<usize>)
where
    T: ?Sized + Bits,
{
    assert_eq!(bits.rank0(r.clone()) + bits.rank1(r.clone()), r.len());
}

#[test]
fn bit_rank() {
    rank_for_empty_range::<u8>(&!0);
    rank_for_empty_range::<[u8]>(&[!0, !0, !0, !0]);

    rank_0_plus_rank_1::<u64>(&0b_1010_1010, 0..10);
    rank_0_plus_rank_1::<u64>(&0b_1010_1010, 7..20);
    rank_0_plus_rank_1::<[u8]>(&[!0, 0b_1010_1010, !0, 0b_1010_1010], 0..10);
    rank_0_plus_rank_1::<[u8]>(&[!0, 0b_1010_1010, !0, 0b_1010_1010], 7..20);
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
    let aux = bits::Pop::from(&vec[..]);
    Bits::bits(&vec) == Bits::bits(&aux)
}

#[quickcheck]
fn repr_rank(vec: Vec<u32>) -> bool {
    let aux = bits::Pop::from(&vec[..]);
    vec.count1() == aux.count1() && vec.count0() == aux.count0()
}

#[quickcheck]
fn repr_select1(vec: Vec<u32>) -> bool {
    let aux = bits::Pop::from(&vec[..]);
    (0..vec.count1()).all(|i| vec.select1(i) == aux.select1(i))
}

#[quickcheck]
fn repr_select0(vec: Vec<u32>) -> bool {
    let aux = bits::Pop::from(&vec[..]);
    (0..vec.count0()).all(|i| vec.select0(i) == aux.select0(i))
}

fn none<T: Block>(n: usize) -> bits::Pop<Vec<T>> {
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

fn check<T: Block + Unpack>(size: usize, bits: Vec<usize>) -> bool {
    let mut aux = none::<T>(size);

    for &b in &bits {
        aux.set1(b);
    }

    assert_eq!(aux.count1(), bits.len());

    bits.into_iter().enumerate().all(|(i, b)| {
        Bits::test(&aux, b).unwrap()
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
