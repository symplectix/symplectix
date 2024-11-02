/// Calculates the minimum number of blocks to store `n` bits.
const fn blocks(n: usize, b: usize) -> usize {
    n / b + (n % b > 0) as usize
}

/// Returns an empty `Vec<T>` with the at least specified capacity in bits.
///
/// # Examples
///
/// ```
/// use bits::Bits;
/// let v = bitvec::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(v.bits(), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: bits::Block>(capacity: usize) -> Vec<T> {
    Vec::with_capacity(blocks(capacity, T::BITS))
}

/// # Examples
///
/// ```
/// use bits::Bits;
/// let v = bitvec::empty::<u8>(80);
/// assert_eq!(v.bits(), 80);
/// assert_eq!(v.len(), 10);
/// ```
pub fn empty<T: bits::Block>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty()))
        .take(blocks(n, T::BITS))
        .collect::<Vec<T>>()
}
