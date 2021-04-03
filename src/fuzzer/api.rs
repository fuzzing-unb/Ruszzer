use crate::runner::api::{Outcome, Runner};
use crate::strategy::api::{Strategy};

pub trait Fuzzer {
    fn run<TRunner: Runner, TStrategy: Strategy>(&self, runner: &TRunner, strategy: &TStrategy) -> Outcome;
    fn runs<TRunner: Runner, TStrategy: Strategy>(&self, runner: &TRunner, strategy: &TStrategy, trials: usize) -> Vec<Outcome>;
}
