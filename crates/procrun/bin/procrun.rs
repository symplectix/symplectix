//! A simple executable that spawns a given commands, waits them and
//! exits gracefully.

use std::env;

fn main() -> procrun::ProcExit {
    procrun::procrun(env::args_os())
}
