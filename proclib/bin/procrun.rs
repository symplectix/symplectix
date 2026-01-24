//! A simple executable that spawns a given commands, waits them and
//! exits gracefully.

use std::env;

fn main() -> proclib::ProcExit {
    proclib::run(env::args_os())
}
