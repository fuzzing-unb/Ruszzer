use crate::runner::api::{CoveredLine, Runner, Outcome};
use crate::mutator::api::Mutator;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use super::api::Strategy;

pub struct MutationStrategy<'a> {
    pub mutator: &'a dyn Mutator,
    pub population: Vec<String>,
    pub runner: &'a dyn Runner,
    pub seeds: Vec<String>,
    pub covered_lines: std::collections::BTreeSet<CoveredLine>,
}

impl <'a> MutationStrategy<'a> {
    pub fn default(mutator: &'a dyn Mutator, runner: &'a dyn Runner) -> MutationStrategy<'a> {
        return MutationStrategy {
            mutator,
            runner,
            seeds: vec![String::from("abcxyz")],
            population: Vec::new(),
            covered_lines: std::collections::BTreeSet::new()
        }
    }

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

impl <'a> Strategy for MutationStrategy<'a>  {

    fn fuzz(&mut self) -> String {
        return if self.seeds.is_empty() {
             self.mutator.mutate(&self.choose_input_from_population())
        }
        else {
            self.seeds.pop().unwrap()
        }
    }

    fn run(&mut self) -> (String, Outcome) {
        let fuzzied_string = self.fuzz();
        let outcome = self.runner.run(&fuzzied_string);
        let mut new_coverages: std::collections::BTreeSet<CoveredLine> = outcome.coverage.covered_lines
            .difference(&self.covered_lines)
            .cloned()
            .collect();
        if !new_coverages.is_empty() {
            println!("New coverages: {}.", new_coverages.len());
        }
        self.population.push(fuzzied_string.clone());
        self.covered_lines.append(&mut new_coverages);
        return (fuzzied_string, outcome);
    }

    fn print_results(&self) {
        println!("Total Coverage: {}", self.covered_lines.len())
    }

}

