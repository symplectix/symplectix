use std::borrow::{
    Cow,
    ToOwned,
};

/// Helper trait for bit masking.
///
/// The mask defines which bits to retain and which to clear.
/// Masking involves applying such a mask to self.
pub trait Masking<Mask: ?Sized = Self> {
    /// Performs inplace and.
    fn and(data: &mut Self, mask: &Mask);

    /// Performs inplace or.
    fn or(data: &mut Self, mask: &Mask);

    /// Performs inplace not.
    fn not(data: &mut Self, mask: &Mask);

    /// Performs inplace xor.
    fn xor(data: &mut Self, mask: &Mask);
}

impl<A, B> Masking<B> for Box<A>
where
    A: ?Sized + Masking<B>,
    B: ?Sized,
{
    #[inline]
    fn and(data: &mut Self, mask: &B) {
        Masking::and(data.as_mut(), mask);
    }
    #[inline]
    fn or(data: &mut Self, mask: &B) {
        Masking::or(data.as_mut(), mask);
    }
    #[inline]
    fn not(data: &mut Self, mask: &B) {
        Masking::not(data.as_mut(), mask);
    }
    #[inline]
    fn xor(data: &mut Self, mask: &B) {
        Masking::xor(data.as_mut(), mask);
    }
}

impl<'a, 'b, A, B> Masking<Cow<'b, B>> for Cow<'a, A>
where
    A: ?Sized + ToOwned,
    B: ?Sized + ToOwned,
    A::Owned: Masking<B>,
{
    #[inline]
    fn and(data: &mut Self, mask: &Cow<'b, B>) {
        Masking::and(data.to_mut(), mask);
    }
    #[inline]
    fn or(data: &mut Self, mask: &Cow<'b, B>) {
        Masking::or(data.to_mut(), mask);
    }
    #[inline]
    fn not(data: &mut Self, mask: &Cow<'b, B>) {
        Masking::not(data.to_mut(), mask);
    }
    #[inline]
    fn xor(data: &mut Self, mask: &Cow<'b, B>) {
        Masking::xor(data.to_mut(), mask);
    }
}

#[cfg(test)]
mod mask_test {
    use std::borrow::Cow;

    use crate::Bits;

    // For testing purposes only. Wrapping integers in a Cow is
    // a waste of space.
    macro_rules! impl_masking_for_word {
        ($( $Word:ty )*) => ($(
            impl crate::Masking<$Word> for $Word {
                #[inline]
                fn and(data: &mut Self, mask: &$Word) {
                    *data &= *mask;
                }
                #[inline]
                fn or(data: &mut Self, mask: &$Word) {
                    *data |= *mask;
                }
                #[inline]
                fn not(data: &mut Self, mask: &$Word) {
                    *data &= !*mask;
                }
                #[inline]
                fn xor(data: &mut Self, mask: &$Word) {
                    *data ^= *mask;
                }
            }
        )*)
    }
    impl_masking_for_word!(u8 u16 u32 u64 u128);

    #[test]
    fn and() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.and(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000100)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b01000000)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn or() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.or(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000101)));
        assert_eq!(iter.next().unwrap(), (1, Cow::Owned(0b11100011)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b01100000)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn not() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.not(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000001)));
        assert_eq!(iter.next().unwrap(), (1, Cow::Owned(0b01100011)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b00100000)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn xor() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.xor(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000001)));
        assert_eq!(iter.next().unwrap(), (1, Cow::Owned(0b11100011)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b00100000)));
        assert_eq!(iter.next(), None);
    }
}
