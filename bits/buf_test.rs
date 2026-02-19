use crate::{
    Bits,
    BitsMut,
    Block,
    Buf,
    Masking,
};

#[test]
fn count1() {
    let b: Buf<[u8; 3]> = Buf::new();
    assert_eq!(b.count1(), 0);
    let b: Buf<[u8; 3]> = Buf::from([0, 1, 0]);
    assert_eq!(b.count1(), 1);
}

#[test]
fn count0() {
    let b: Buf<[u8; 3]> = Buf::new();
    assert_eq!(b.count0(), 24);
    let b: Buf<[u8; 3]> = Buf::from([0, 0, 0]);
    assert_eq!(b.count0(), 24);
}

#[test]
fn all() {
    let b: Buf<[u8; 3]> = Buf::new();
    assert_eq!(b.count0(), 24);
    let b: Buf<[u8; 3]> = Buf::from([0, 0, 0]);
    assert_eq!(b.count0(), 24);
}

#[test]
fn any() {
    let b: Buf<[u8; 3]> = Buf::new();
    assert!(!b.any());
    let b: Buf<[u8; 3]> = Buf::from([0, 0, 0]);
    assert!(!b.any());
    let b: Buf<[u8; 3]> = Buf::from([0, 1, 0]);
    assert!(b.any());
}

#[test]
fn bit() {
    let b: Buf<[u8; 3]> = Buf::new();
    assert!(!b.bit(8));
    let b: Buf<[u8; 3]> = Buf::from([0, 1, 0]);
    assert!(b.bit(8));
}

#[test]
fn word() {
    let b: Buf<[u8; 3]> = Buf::new();
    assert_eq!(b.word::<u8>(0, 3), 0b_000);
    let b: Buf<[u8; 3]> = Buf::from([1, 1, 1]);
    assert_eq!(b.word::<u8>(0, 3), 0b_001);
}

#[test]
fn rank1() {
    let b: Buf<[u8; 3]> = Buf::new();
    assert_eq!(b.rank1(..10), 0);
    let b: Buf<[u8; 3]> = Buf::from([0, 1, 0]);
    assert_eq!(b.rank1(..10), 1);
}

#[test]
fn rank0() {
    let b: Buf<[u8; 3]> = Buf::new();
    assert_eq!(b.rank0(..10), 10);
    let b: Buf<[u8; 3]> = Buf::from([0, 1, 0]);
    assert_eq!(b.rank0(..10), 9);
}

#[test]
fn select1_select0() {
    let b: Buf<[u8; 3]> = Buf::new();
    assert_eq!(b.select1(0), None);
    let b: Buf<[u8; 3]> = Buf::from([0, 1, 0]);
    assert_eq!(b.select1(0), Some(8));

    let b: Buf<[u8; 3]> = Buf::new();
    assert_eq!(b.select0(0), Some(0));
    assert_eq!(b.select0(100), None);
    let b: Buf<[u8; 3]> = Buf::from([0, 1, 0]);
    assert_eq!(b.select0(10), Some(11));

    let mut b = Buf::<[u64; 8]>::empty();
    assert_eq!(b.select1(0), None);
    assert_eq!(b.select0(0), Some(0));
    assert_eq!(b.select0(Buf::<[u64; 8]>::BITS - 1), Some(511));
    b.set1(1);
    b.set1(511);
    assert_eq!(b.select1(0), Some(1));
    assert_eq!(b.select1(1), Some(511));
    assert_eq!(b.select0(0), Some(0));
    assert_eq!(b.select0(1), Some(2));
}

#[test]
fn intersection() {
    let mut a = Buf::<[u64; 4]>::new();
    a.set1(0);
    a.set1(1);
    a.set1(2);
    a.set1(128);
    let mut b = Buf::<[u64; 4]>::new();
    b.set1(1);
    b.set1(2);
    b.set1(3);
    b.set1(192);
    a.intersection(&b);
    assert_eq!(a.as_ref().unwrap(), &[0b_0110, 0, 0, 0]);
    let mut c = Buf::<[u64; 4]>::new();
    b.intersection(&c);
    assert_eq!(b.as_ref(), None);
    c.intersection(&a);
    assert_eq!(c.as_ref(), None);
}

#[test]
fn union() {
    let mut a = Buf::<[u64; 4]>::new();
    a.set1(0);
    a.set1(1);
    a.set1(2);
    a.set1(128);
    let mut b = Buf::<[u64; 4]>::new();
    b.set1(1);
    b.set1(2);
    b.set1(3);
    b.set1(192);
    a.union(&b);
    assert_eq!(a.as_ref().unwrap(), &[0b_1111, 0, 1, 1]);
    let mut c = Buf::<[u64; 4]>::new();
    c.union(&b);
    assert_eq!(c.as_ref().unwrap(), &[0b_1110, 0, 0, 1]);
}

#[test]
fn difference() {
    let mut a = Buf::<[u64; 4]>::new();
    a.set1(0);
    a.set1(1);
    a.set1(2);
    a.set1(128);

    let mut b = Buf::<[u64; 4]>::new();
    b.set1(1);
    b.set1(2);
    b.set1(3);
    b.set1(192);

    a.difference(&b);
    assert_eq!(a.as_ref().unwrap(), &[0b_0001, 0, 1, 0]);

    let mut c = Buf::<[u64; 4]>::new();
    c.difference(&a);
    assert_eq!(c.as_ref(), None);
}

#[test]
fn symmetric_difference() {
    let mut a = Buf::<[u64; 4]>::new();
    a.set1(0);
    a.set1(1);
    a.set1(2);
    a.set1(128);
    let mut b = Buf::<[u64; 4]>::new();
    b.set1(1);
    b.set1(2);
    b.set1(3);
    b.set1(192);
    a.symmetric_difference(&b);
    assert_eq!(a.as_ref().unwrap(), &[0b_1001, 0, 1, 1]);
    let mut c = Buf::<[u64; 4]>::new();
    c.symmetric_difference(&a);
    assert_eq!(c.as_ref().unwrap(), &[0b_1001, 0, 1, 1]);
}
