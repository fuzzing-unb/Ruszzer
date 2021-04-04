
use crate::runner::api::{CoveredLine, Runner, Outcome};
use crate::strategy::mutation_strategy::MutationStrategy;
use crate::strategy::api::Strategy;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct GreyboxStrategy {
    pub mutator: MutationStrategy,
    pub covered_lines: std::collections::BTreeSet<CoveredLine>,
    pub population: Vec<String>,
}

impl GreyboxStrategy {

    fn choose_input_from_population(&self) -> String {
        if self.population.is_empty() {
            panic!();
        }
        let weights: Vec<u32> = self.population.clone().into_iter().map(|_| 1).collect();
        let mut rng = thread_rng();
        let dist = WeightedIndex::new(&weights).unwrap();
        let choice = &self.population[dist.sample(&mut rng)];
        return choice.clone();
    }
}
impl Strategy for GreyboxStrategy {
    fn fuzz(&self) -> String {
        let input = if self.population.is_empty() {
            self.mutator.seed.clone()
        }
        else {
            self.choose_input_from_population()
        };
        return self.mutator.mutate(&input);
    }

    fn run<TRunner: Runner>(&mut self, runner: &TRunner) -> Outcome {
        let fuzzied_string = self.fuzz();
        let outcome = runner.run(&fuzzied_string);
        let mut new_coverages: std::collections::BTreeSet<CoveredLine> = outcome.coverage.covered_lines.difference(&self.covered_lines)
                                                                                                        .cloned()
                                                                                                        .collect();
        if !new_coverages.is_empty() {
            println!("New coverages: {}.", new_coverages.len());
            self.population.push(fuzzied_string);
            self.covered_lines.append(&mut new_coverages);
        }
        return outcome;
    }
}