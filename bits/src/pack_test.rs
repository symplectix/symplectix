#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use bits::block::*;
use bits::BitVec;
use rand::prelude::*;

fn pack_unpacked_imp<const N: usize>(orig: Vec<u32>) {
    let mut bits = BitVec::from(orig.clone());

    for i in 0..bits.bits() {
        let unpacked = bits.unpack::<u8>(i, N);
        bits.pack(i, N, unpacked);
        // visible in console if error happens
        assert_eq!(orig, bits.as_slice());
    }
}

#[quickcheck]
fn pack_unpacked(orig: Vec<u32>) {
    pack_unpacked_imp::<7>(orig);
}

fn unpack_packed_imp<const N: usize>(values: Vec<u16>, mut rng: impl Rng) {
    let mut bits = BitVec::<u64>::new(65536);

    for val in values {
        let i = rng.gen_range(0..65536 - N);
        bits.pack(i, N, val);
        let unpacked = bits.unpack::<u16>(i, N);

        if unpacked != (val << (16 - N)) >> (16 - N) {
            dbg!(i, N, val, unpacked, (val as u64) << (16 - N) >> (16 - N));
        }
        assert_eq!(unpacked, (val << (16 - N)) >> (16 - N));
    }
}

#[quickcheck]
fn unpack_packed(orig: Vec<u16>) {
    let mut rng = rand::thread_rng();
    unpack_packed_imp::<14>(orig, &mut rng);
}

#[test]
fn unpack() {
    let bits: u64 = 0b_10111111_10101100_10101011_01010101_01010101;
    assert_eq!(bits.unpack::<u8>(0, 0), 0b_00000);
    assert_eq!(bits.unpack::<u8>(0, 1), 0b_00001);
    assert_eq!(bits.unpack::<u8>(1, 0), 0b_00000);
    assert_eq!(bits.unpack::<u8>(1, 1), 0b_00000);
    assert_eq!(bits.unpack::<u8>(1, 2), 0b_00010);
    assert_eq!(bits.unpack::<u8>(3, 5), 0b_01010);
    assert_eq!(bits.unpack::<u8>(6, 5), 0b_10101);
    assert_eq!(bits.unpack::<u8>(6, 10), 0b_0101_0101);

    assert_eq!(bits.unpack::<u64>(0, 64), bits);
}

#[test]
fn pack() {
    let orig = vec![1, 0b_10101100_10101011_01010101_01010101_u32];
    let mut bits = BitVec::from(orig.clone());
    let unpacked = bits.unpack::<u8>(1, 7);
    assert_eq!(unpacked, 0);
    bits.pack(1, 7, unpacked);
    assert_eq!(bits.unpack::<u8>(1, 7), 0);
    assert_eq!(orig, bits.as_slice());

    let mut bits: u64 = 0b_10111111_10101100_10101011_01010101_01010101;
    bits.pack::<u8>(0, 3, 0b_1010);
    assert_eq!(bits.unpack::<u8>(0, 4), 0b_0010);
    assert_eq!(bits.unpack::<u64>(0, 64), 0b_10111111_10101100_10101011_01010101_01010010);
    assert_eq!(bits.unpack::<u64>(1, 64), 0b_01011111_11010110_01010101_10101010_10101001);

    bits.pack::<u8>(7, 3, 0b_000);
    assert_eq!(bits.unpack::<u8>(7, 3), 0b_000);
    assert_eq!(bits.unpack::<u8>(63, 3), 0b_000);
}
