use super::api::Fuzzer;
use crate::runner::api::{Runner, Outcome};
use crate::strategy::api::{Strategy};

pub struct RandomFuzzer {
}

impl RandomFuzzer {
    pub fn default() -> RandomFuzzer {
        return RandomFuzzer { }
    }
} 

impl Fuzzer for RandomFuzzer {

    fn run<TRunner: Runner, TStrategy: Strategy>(&self, runner: &TRunner, strategy: &TStrategy) -> Outcome {
        let fuzzied_string = strategy.fuzz();
        return runner.run(&fuzzied_string);
    }

    fn runs<TRunner: Runner, TStrategy: Strategy>(&self, runner: &TRunner, strategy: &TStrategy, trials: usize) -> Vec<Outcome> {
        let mut vec = Vec::with_capacity(trials);
        for _trial in 1..=trials {
            vec.push(self.run(runner, strategy));
        }
        return vec;
    }

}
