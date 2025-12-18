use std::marker::PhantomData;
use std::ops::AddAssign;

use crate::{ControlFlow, Fold};

#[derive(Debug)]
pub struct Sum<A, B> {
    _item: PhantomData<A>,
    _acc: PhantomData<B>,
}

impl<A, T> Sum<A, T> {
    pub fn new() -> Self {
        Sum { _item: PhantomData, _acc: PhantomData }
    }
}

impl<A, T> Fold<A, T> for Sum<A, T>
where
    T: AddAssign<A>,
{
    type Acc = T;

    #[inline]
    fn step(&mut self, mut acc: Self::Acc, item: A) -> ControlFlow<Self::Acc> {
        use std::ops::ControlFlow::Continue;
        acc += item;
        Continue(acc)
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        acc
    }
}
