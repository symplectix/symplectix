//! Defines internal (yet) traits and types.

/// A fold step function.
pub trait StepFn<T> {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    fn step(&mut self, acc: Self::Acc, input: T) -> Step<Self::Acc>;

    /// Invoked when folding is complete.
    /// By default, done just returns acc.
    ///
    /// You must call `done` exactly once.
    ///
    /// ```compile_fail
    /// # use mung::StepFn;
    /// # struct SomeStepFn();
    /// # impl StepFn<i32> for SomeStepFn {
    /// #     type Acc = usize;
    /// #     fn step(&mut self, mut acc: Self::Acc, _i: i32) -> mung::Step<Self::Acc> {
    /// #         mung::Step::Yield(acc + 1)
    /// #     }
    /// # }
    /// let f = SomeStepFn();
    /// f.done(0);
    /// f.done(0);
    /// ```
    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc
    where
        Self: Sized,
    {
        acc
    }
}

/// The result of [Fold.step].
#[derive(Debug, Copy, Clone)]
pub enum Step<T> {
    /// Keep folding.
    Yield(T),
    /// Stop folding.
    Break(T),
}

/// An adapter that creates a new [StepFn] from the given one.
pub trait Xform<Sf> {
    /// A new step function created by apply.
    type StepFn;

    /// Creates a new [StepFn] from the given one.
    fn apply(self, step_fn: Sf) -> Self::StepFn;
}
