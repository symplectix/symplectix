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
fn masking_and() {
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
    Masking::and(&mut a, &b);
    assert_eq!(a.as_ref().unwrap(), &[0b_0110, 0, 0, 0]);
    let mut c = Buf::<[u64; 4]>::new();
    Masking::and(&mut b, &c);
    assert_eq!(b.as_ref(), None);
    Masking::and(&mut c, &a);
    assert_eq!(c.as_ref(), None);
}

#[test]
fn masking_or() {
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
    Masking::or(&mut a, &b);
    assert_eq!(a.as_ref().unwrap(), &[0b_1111, 0, 1, 1]);
    let mut c = Buf::<[u64; 4]>::new();
    Masking::or(&mut c, &b);
    assert_eq!(c.as_ref().unwrap(), &[0b_1110, 0, 0, 1]);
}

#[test]
fn masking_not() {
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

    Masking::not(&mut a, &b);
    assert_eq!(a.as_ref().unwrap(), &[0b_0001, 0, 1, 0]);

    let mut c = Buf::<[u64; 4]>::new();
    Masking::not(&mut c, &a);
    assert_eq!(c.as_ref(), None);
}

#[test]
fn masking_xor() {
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
    Masking::xor(&mut a, &b);
    assert_eq!(a.as_ref().unwrap(), &[0b_1001, 0, 1, 1]);
    let mut c = Buf::<[u64; 4]>::new();
    Masking::xor(&mut c, &a);
    assert_eq!(c.as_ref().unwrap(), &[0b_1001, 0, 1, 1]);
}
