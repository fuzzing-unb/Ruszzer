use crate::runner::runner::{Outcome, Runner};

pub trait Fuzzer {
    fn fuzz(&self) -> String;
    fn run(&self, runner: &dyn Runner) -> Outcome;
    fn runs(&self, runner: &dyn Runner, trials: usize) -> Vec<Outcome>;
}
