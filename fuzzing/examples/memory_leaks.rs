#![no_main]

use libfuzzer_sys::{fuzz_target, Corpus};

fuzz_target!(|data: &[u8]| -> Corpus {
    if data.is_empty() {
        return Corpus::Reject;
    }

    let data = Vec::from(data);
    std::mem::forget(data);

    Corpus::Keep
});
