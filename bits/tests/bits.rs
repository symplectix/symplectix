use bits::{Count, Select, Word};
use std::borrow::Cow;
use std::iter::successors;

#[test]
fn bits_is_implemented() {
    fn _test<T>()
    where
        T: ?Sized + bits::Bits + bits::Count + bits::Rank + bits::Select,
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

fn ones<T: Word>(word: T) -> impl Iterator<Item = usize> {
    successors(Some(word), |&n| {
        let m = n & !n.lsb();
        m.any().then(|| m)
    })
    .map(Word::count_t0)
}

#[test]
fn next_set_bit() {
    let n: u32 = 0b_0101_0101;
    let mut ones = ones(n);

    assert_eq!(ones.next(), Some(0));
    assert_eq!(ones.next(), Some(2));
    assert_eq!(ones.next(), Some(4));
    assert_eq!(ones.next(), Some(6));
    assert_eq!(ones.next(), None);
}

#[test]
fn ones_select1() {
    let n: u32 = 0b_0101_0101;
    let mut ones = ones(n);
    for c in 0..n.count1() {
        assert_eq!(ones.next(), n.select1(c));
    }
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
    rank_for_empty_range::<[bool]>(&[true, true, true, true, true, true, true, true]);

    rank_0_plus_rank_1::<u64>(&0b_1010_1010, 0..10);
    rank_0_plus_rank_1::<u64>(&0b_1010_1010, 7..20);
    rank_0_plus_rank_1::<[u8]>(&[!0, 0b_1010_1010, !0, 0b_1010_1010], 0..10);
    rank_0_plus_rank_1::<[u8]>(&[!0, 0b_1010_1010, !0, 0b_1010_1010], 7..20);
}
