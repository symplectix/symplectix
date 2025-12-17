use std::marker::PhantomData;

use super::Xform;
use crate::Fold;

#[derive(Debug)]
pub struct Identity<A, B>(pub(crate) PhantomData<(A, B)>);

impl<A, B> Identity<A, B> {
    pub(crate) fn new() -> Self {
        Identity(PhantomData)
    }
}

impl<A, B, Sf: Fold<A, B>> Xform<Sf> for Identity<A, B> {
    type Fold = Sf;
    #[inline]
    fn xform(self, step_fn: Sf) -> Self::Fold {
        step_fn
    }
}
