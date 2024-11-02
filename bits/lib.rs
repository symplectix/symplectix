//! `bits`

macro_rules! mask {
    ($( $Int: ty, $i: expr, $j: expr )*) => ($(
        if $i >= $j {
            0
        } else {
            !0 >> (<$Int>::BITS as usize - ($j - $i)) << $i
        }
    )*)
}

mod bits;
mod container;
mod container_mut;
mod count;
mod excess;
mod lsb;
mod msb;
mod rank;
mod select;

mod mask;

pub mod and;
pub mod not;
pub mod or;
pub mod xor;

pub use self::bits::Bits;
pub use self::container::Container;
pub use self::container_mut::ContainerMut;
pub use self::count::Count;
pub use self::excess::Excess;
pub use self::mask::Mask;
pub use self::rank::Rank;
pub use self::select::Select;
pub use self::{lsb::Lsb, msb::Msb};
