use super::api::Fuzzer;
use crate::runner::api::{Runner, Outcome, CoveredLine};
use crate::strategy::api::{Strategy};

pub struct FuzzerImpl {
    pub covered_lines: std::collections::BTreeSet<CoveredLine>,
}

impl FuzzerImpl {
    pub fn default() -> FuzzerImpl {
        return FuzzerImpl {
            covered_lines: std::collections::BTreeSet::new()
         }
    }
} 

impl Fuzzer for FuzzerImpl {

    fn run<TRunner: Runner, TStrategy: Strategy>(&mut self, runner: &TRunner, strategy: &TStrategy) -> Outcome {
        let fuzzied_string = strategy.fuzz();
        let outcome = runner.run(&fuzzied_string);
        let mut new_coverages: std::collections::BTreeSet<CoveredLine> = outcome.coverage.covered_lines.difference(&self.covered_lines)
                                                                                                        .cloned()
                                                                                                        .collect();
        if !new_coverages.is_empty() {
            println!("New coverages: {}.", new_coverages.len());
        }
        self.covered_lines.append(&mut new_coverages);
        return outcome;
    }

    fn runs<TRunner: Runner, TStrategy: Strategy>(&mut self, runner: &TRunner, strategy: &TStrategy, trials: usize) -> Vec<Outcome> {
        let mut vec = Vec::with_capacity(trials);
        for _trial in 1..=trials {
            vec.push(self.run(runner, strategy));
        }
        return vec;
    }

    fn total_coverage(&self) -> usize {
        return self.covered_lines.len();
    }

}
