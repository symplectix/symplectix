use crate::Xform;

#[derive(Debug)]
pub struct Comp<F, G> {
    f: F,
    g: G,
}

impl<F, G> Comp<F, G> {
    pub(crate) fn new(f: F, g: G) -> Self {
        Comp { f, g }
    }
}

impl<Sf, F, G> Xform<Sf> for Comp<F, G>
where
    F: Xform<G::Fold>,
    G: Xform<Sf>,
{
    type Fold = F::Fold;

    fn xform(self, rf: Sf) -> Self::Fold {
        self.f.xform(self.g.xform(rf))
    }
}
