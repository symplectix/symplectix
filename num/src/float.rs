pub trait Float: Sized + Copy + PartialEq + PartialOrd {}

macro_rules! impl_float {
    ($( $N:ty )*) => ($(
        impl Float for $N {}
    )*)
}
impl_float!(f32 f64);
