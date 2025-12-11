#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [transducers](https://clojure.org/reference/transducers)
// - [cgrand/xforms]: https://github.com/cgrand/xforms
// - [foldl](https://hackage.haskell.org/package/foldl)

mod filter;
mod map;
mod xfn;

pub use filter::Filter;
pub use map::Map;

#[derive(Debug)]
pub struct Fold<Sf> {
    sf: Sf,
}

#[derive(Debug)]
pub struct Prep<Xf> {
    xf: Xf,
}

impl<Sf> Fold<Sf> {
    fn step<T>(&mut self, acc: Sf::Acc, input: T) -> xfn::Step<Sf::Acc>
    where
        Sf: xfn::StepFn<T>,
    {
        self.sf.step(acc, input)
    }

    fn done<T>(self, acc: Sf::Acc) -> Sf::Acc
    where
        Sf: xfn::StepFn<T>,
    {
        self.sf.done(acc)
    }
}

impl<Xf> Prep<Xf> {
    pub fn apply<Sf>(self, step_fn: Sf) -> Fold<Xf::StepFn>
    where
        Xf: xfn::Xform<Sf>,
    {
        Fold { sf: self.xf.apply(step_fn) }
    }

    pub(crate) fn new(xf: Xf) -> Self {
        Prep { xf }
    }

    pub fn map<F>(self, f: F) -> Prep<Comp<Xf, Map<F>>> {
        Prep::new(comp(self.xf, Map::new(f)))
    }

    pub fn filter<P>(self, predicate: P) -> Prep<Comp<Xf, Filter<P>>> {
        Prep::new(comp(self.xf, Filter::new(predicate)))
    }
}

pub fn map<F>(f: F) -> Prep<Map<F>> {
    Prep::new(Map::new(f))
}

pub fn filter<P>(predicate: P) -> Prep<Filter<P>> {
    Prep::new(Filter::new(predicate))
}

fn comp<A, B>(a: A, b: B) -> Comp<A, B> {
    Comp { a, b }
}

/// Comp is an adapter of [Adapter]s.
pub struct Comp<A, B> {
    a: A,
    b: B,
}

impl<Fl, A, B> xfn::Xform<Fl> for Comp<A, B>
where
    A: xfn::Xform<B::StepFn>,
    B: xfn::Xform<Fl>,
{
    type StepFn = A::StepFn;

    fn apply(self, rf: Fl) -> Self::StepFn {
        self.a.apply(self.b.apply(rf))
    }
}

#[cfg(test)]
mod tests {
    use super::xfn;

    struct PushVec;

    impl<T> xfn::StepFn<T> for PushVec {
        type Acc = Vec<T>;

        fn step(&mut self, mut acc: Self::Acc, v: T) -> xfn::Step<Self::Acc> {
            acc.push(v);
            xfn::Step::Yield(acc)
        }
    }

    #[test]
    fn test_map_filter_step() {
        let mut acc = vec![];
        let mut fold = super::map(|x| x * 2 + 1).filter(|x: &i32| 10 < *x && *x < 20).apply(PushVec);
        for i in 0..20 {
            match fold.step(acc, i) {
                xfn::Step::Yield(ret) => {
                    acc = ret;
                }
                xfn::Step::Break(ret) => {
                    acc = fold.done(ret);
                    break;
                }
            }
        }
        assert_eq!(acc, vec![11, 13, 15, 17, 19]);
    }
}
