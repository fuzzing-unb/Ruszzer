use crate::runner::runner::{Output, Runner};

pub trait Fuzzer {
    fn fuzz(&self) -> String;
    fn run(&self, runner: &dyn Runner) -> Output;
    fn runs(&self, runner: &dyn Runner, trials: usize) -> Vec<Output>;
}
