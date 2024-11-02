#![allow(dead_code)] // TODO: REMOVE
#![allow(unused_imports)] // TODO: REMOVE

mod l1l2;
mod rank_aux;

use bits::Container;
use l1l2::L1L2;
// use rank_aux::{Buckets, Uninit};
// use rank_aux::{Pop as L1L2Sum, Rho as L1L2Bit};

pub use rank_aux::Rho;

/// Calculates the minimum number of blocks to store `n` bits.
const fn blocks(n: usize, b: usize) -> usize {
    n / b + (n % b > 0) as usize
}

/// Returns an empty `Vec<T>` with the at least specified capacity in bits.
///
/// # Examples
///
/// ```
/// use bits::Container;
/// let v = bitvec::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(v.bits(), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: bits::Bits>(capacity: usize) -> Vec<T> {
    Vec::with_capacity(blocks(capacity, T::BITS))
}

/// # Examples
///
/// ```
/// use bits::Container;
/// let v = bitvec::empty::<u8>(80);
/// assert_eq!(v.bits(), 80);
/// assert_eq!(v.len(), 10);
/// ```
pub fn empty<T: bits::Bits>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty())).take(blocks(n, T::BITS)).collect::<Vec<T>>()
}

// // impl<T: Container, S> From<Imp<Box<[T]>, S>> for Imp<Vec<T>, S> {
// //     #[inline]
// //     fn from(imp: Imp<Box<[T]>, S>) -> Imp<Vec<T>, S> {
// //         Imp { buckets: imp.buckets, samples: imp.samples, bit_vec: imp.bit_vec.into_vec() }
// //     }
// // }
// // impl<T: Container, S> From<Imp<Vec<T>, S>> for Imp<Box<[T]>, S> {
// //     #[inline]
// //     fn from(imp: Imp<Vec<T>, S>) -> Imp<Box<[T]>, S> {
// //         Imp { buckets: imp.buckets, samples: imp.samples, bit_vec: imp.bit_vec.into_boxed_slice() }
// //     }
// // }

// // fn sbs_from_heaps<T: WordArray>(slice: &[Block<T>]) -> impl Iterator<Item = Option<&[T::Elem]>> {
// //     assert!(Block::<T>::BITS % SUPER == 0 && SUPER <= 65536);
// //     type BoxIter<'a, A> = Box<dyn Iterator<Item = Option<A>> + 'a>;

// //     slice.iter().flat_map(move |heap| {
// //         heap.as_slice().map_or_else(
// //             || {
// //                 use std::iter::repeat;
// //                 Box::new(repeat(None).take(Block::<T>::BITS / SUPER)) as BoxIter<'_, &'_ [T::Elem]>
// //             },
// //             |b| Box::new(sbs_from_words(b)) as BoxIter<'_, &'_ [T::Elem]>,
// //         )
// //     })
// // }
