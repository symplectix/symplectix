#![no_main]
#![allow(unused_comparisons)]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    assert!(0 <= data.len());
});
