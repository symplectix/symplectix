//! Just print buildinfo collected at compile time.

include!(concat!(env!("OUT_DIR"), "/buildinfo.rs"));

fn main() {
    let x = concat!(env!("OUT_DIR"), "/buildinfo");
    println!("{:?}", x);
    println!("FOO: {}", buildinfo::FOO);
}
