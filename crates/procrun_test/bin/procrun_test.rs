#![allow(missing_docs)]

use std::process;

fn main() -> procrun::Exit {
    println!("procrun_orphan: pid={}", process::id());
    procrun::run_args(["procrun", procrun_test::ORPHAN_BIN])
}
