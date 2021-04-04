
use crate::runner::api::{Outcome, Runner};

pub trait Strategy {
    fn fuzz(&self) -> String;

    fn run<TRunner: Runner>(&mut self, runner: &TRunner) -> Outcome;

    fn runs<TRunner: Runner>(&mut self, runner: &TRunner, trials: usize) -> Vec<Outcome> {
        let mut vec = Vec::with_capacity(trials);
        for _trial in 1..=trials {
            vec.push(self.run(runner));
        }
        return vec;
    }
}
