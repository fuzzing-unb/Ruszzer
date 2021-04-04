use crate::runner::api::{CoveredLine, Runner, Outcome};
use crate::mutator::api::Mutator;

use super::api::Strategy;

pub struct MutationStrategy<'a> {
    pub mutator: &'a dyn Mutator,
    pub seed: String,
    pub covered_lines: std::collections::BTreeSet<CoveredLine>,
}

impl <'a> MutationStrategy<'a> {
    pub fn default(mutator: &'a dyn Mutator) -> MutationStrategy<'a> {
        return MutationStrategy {
            mutator,
            seed: String::from(""),
            covered_lines: std::collections::BTreeSet::new()
        }
    }
}

impl <'a> Strategy for MutationStrategy<'a>  {

    fn fuzz(&self) -> String {
        return self.mutator.mutate(&self.seed);
    }

    fn run<TRunner: Runner>(&mut self, runner: &TRunner) -> Outcome {
        let fuzzied_string = self.fuzz();
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
    
}

