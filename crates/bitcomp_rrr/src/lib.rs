#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod rrr15;

#[cfg(test)]
mod rrr04;
#[cfg(test)]
mod rrr31;
#[cfg(test)]
mod rrr63;

#[cfg(test)]
mod rrr04_test;
#[cfg(test)]
mod rrr15_test;
#[cfg(test)]
mod rrr31_test;
#[cfg(test)]
mod rrr63_test;

pub use rrr15::*;
