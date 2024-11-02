#![no_main]

use std::thread;

use libfuzzer_sys::{fuzz_target, Corpus};

fuzz_target!(|data: &[u8]| -> Corpus {
    if data.is_empty() {
        return Corpus::Reject;
    }

    static mut ANSWER: i32 = 0;
    let t1 = thread::spawn(|| unsafe { ANSWER = 42 });
    unsafe {
        ANSWER = 24;
    }
    t1.join().ok();
    Corpus::Keep
});
