use std::borrow::{
    Cow,
    ToOwned,
};
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::iter::{
    Fuse,
    Peekable,
};

use crate::{
    Block,
    IntoBlocks,
};
