use crate::runner::api::{Outcome, Runner};
use crate::strategy::api::{Strategy};

pub trait Fuzzer {
    fn total_coverage(&self) -> usize;
    fn run<TRunner: Runner, TStrategy: Strategy>(&mut self, runner: &TRunner, strategy: &TStrategy) -> Outcome;
    fn runs<TRunner: Runner, TStrategy: Strategy>(&mut self, runner: &TRunner, strategy: &TStrategy, trials: usize) -> Vec<Outcome>;
}
