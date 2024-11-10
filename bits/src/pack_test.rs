#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use bits::Bits;
use bits_pack::{Pack, Unpack};

#[quickcheck]
fn packing_unpacked_bits(mut data: Vec<u32>) -> bool {
    let orig = data.clone();
    let len = 7;
    (0..Bits::bits(&data)).all(|n| {
        data.pack(n, len, data.unpack::<u8>(n, len));
        orig == data
    })
}
