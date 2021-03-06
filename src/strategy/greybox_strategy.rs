
use crate::runner::api::{CoveredLine, Runner, Outcome};
use crate::strategy::api::Strategy;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use crate::mutator::api::Mutator;

pub struct GreyboxStrategy<'a> {
    pub mutator: &'a dyn Mutator,
    pub runner: &'a dyn Runner,
    pub seeds: Vec<String>,
    pub covered_lines: std::collections::BTreeSet<CoveredLine>,
    pub population: Vec<String>,
}

impl <'a> GreyboxStrategy <'a> {

    pub fn default(mutator: &'a dyn Mutator, runner: &'a dyn Runner) -> GreyboxStrategy<'a> {
        return GreyboxStrategy {
            mutator,
            runner,
            seeds: vec![String::from("abcxyz")],
            covered_lines: std::collections::BTreeSet::new(),
            population: Vec::new()
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
impl <'a> Strategy for GreyboxStrategy<'a> {

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
            println!("New coverages: {}. String: {}", new_coverages.len(), fuzzied_string);
            self.population.push(fuzzied_string.clone());
            self.covered_lines.append(&mut new_coverages);
        }
        return (fuzzied_string, outcome);
    }

    fn print_results(&self) {
        println!("Total Coverage: {} - Population: {:?}", self.covered_lines.len(), self.population);
    }

}