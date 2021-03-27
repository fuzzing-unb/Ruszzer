use crate::runner::api::{Outcome, Runner};

pub trait Mutator {
    fn mutate(&self, target: &String) -> String;
    fn run(&self, runner: &dyn Runner, seed: String) -> Outcome;
    fn runs(&self, runner: &dyn Runner, seed: String, trials: usize) -> Vec<Outcome>;
}
