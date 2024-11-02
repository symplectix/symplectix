#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use bits::{Container, Count, Lsb, Rank, Select};
use std::borrow::Cow;
use std::iter::successors;

#[test]
fn bits_is_implemented() {
    fn _test<T>()
    where
        T: ?Sized + bits::Container + bits::Count + bits::Rank + bits::Select,
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
}

#[quickcheck]
fn lsb(u: u32) -> bool {
    let i = u as i32;
    u.lsb() == (i & -i) as u32
}

#[quickcheck]
fn next_set_bit(n: u32) -> bool {
    let mut set_bit = successors(Some(n), |&n| {
        let m = n & !n.lsb();
        m.any().then_some(m)
    })
    .map(|x| u32::trailing_zeros(x) as usize);

    for c in 0..n.count1() {
        assert_eq!(set_bit.next(), n.select1(c));
    }

    true
}

#[quickcheck]
fn rank_count(vec: Vec<u32>) -> bool {
    vec.count0() == vec.rank0(..) && vec.count1() == vec.rank1(..)
}

#[quickcheck]
fn bits_rank0_rank1(vec: Vec<u32>) -> bool {
    vec.bits() == vec.rank1(..) + vec.rank0(..)
}

fn rank_for_empty_range<T>(bits: &T)
where
    T: ?Sized + bits::Rank,
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
    T: ?Sized + bits::Rank,
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
    let aux = bitaux::BitAux::from(&vec[..]);
    vec.bits() == aux.bits()
}

#[quickcheck]
fn repr_rank(vec: Vec<u32>) -> bool {
    let aux = bitaux::BitAux::from(&vec[..]);
    vec.count1() == aux.count1() && vec.count0() == aux.count0()
}

#[quickcheck]
fn repr_select1(vec: Vec<u32>) -> bool {
    let aux = bitaux::BitAux::from(&vec[..]);
    (0..vec.count1()).all(|i| vec.select1(i) == aux.select1(i))
}

#[quickcheck]
fn repr_select0(vec: Vec<u32>) -> bool {
    let aux = bitaux::BitAux::from(&vec[..]);
    (0..vec.count0()).all(|i| vec.select0(i) == aux.select0(i))
}
