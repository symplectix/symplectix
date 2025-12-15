#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [foldl](https://github.com/Gabriella439/foldl)
// - [prefolds](https://github.com/effectfully/prefolds)
// - [transducers](https://clojure.org/reference/transducers)
// - [xforms](https://github.com/cgrand/xforms)

// xforms
mod fold;
pub mod xf;

pub use fold::{Step, StepFn};
