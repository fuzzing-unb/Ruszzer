use crate::runner::api::{CoveredLine, Runner, Outcome};
use crate::mutator::api::Mutator;
use crate::strategy::api::Strategy;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

type PathIdentifier = u64;

pub struct BoostedGreyboxStrategy<'a> {
    pub mutator: &'a dyn Mutator,
    pub runner: &'a dyn Runner,
    pub seeds: Vec<String>,
    pub covered_lines: std::collections::BTreeSet<CoveredLine>,
    pub population: Vec<(String, PathIdentifier)>,
    pub path_frequency_map: std::collections::BTreeMap<PathIdentifier, u64>,
    pub exponential_power_schedule: u32
}

impl <'a> BoostedGreyboxStrategy <'a> {

    pub fn default(mutator: &'a dyn Mutator, runner: &'a dyn Runner) -> BoostedGreyboxStrategy<'a> {
        return BoostedGreyboxStrategy {
            mutator,
            runner,
            seeds: vec![String::from("abcxyz")],
            covered_lines: std::collections::BTreeSet::new(),
            population: Vec::new(),
            path_frequency_map: std::collections::BTreeMap::new(),
            exponential_power_schedule: 1
        }
    }

    fn choose_input_from_population(&self) -> String {
        if self.population.is_empty() {
            panic!();
        }
        let weights: Vec<f64> = self.population.clone().into_iter().map(|x| self.compute_energy(x.1)).collect();
        let mut rng = thread_rng();
        let dist = WeightedIndex::new(&weights).unwrap();
        let choice = &self.population[dist.sample(&mut rng)];
        return choice.0.clone();
    }

    fn compute_energy(&self, id: PathIdentifier) -> f64 {
        let path_frequency = self.path_frequency_map.get(&id).unwrap();
        let energy = 1.0 / ((path_frequency.pow(self.exponential_power_schedule)) as f64);
        return energy;
    }
}


impl <'a> Strategy for BoostedGreyboxStrategy<'a> {

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

        let covered_lines = &outcome.coverage.covered_lines;
        let mut hasher = DefaultHasher::new();
        covered_lines.hash(&mut hasher);
        let path_identifier = hasher.finish();
        let path_frequency = self.path_frequency_map.get(&path_identifier).unwrap_or(&0);
        let path_frequency = path_frequency + 1;
        self.path_frequency_map.insert(path_identifier, path_frequency);

        let mut new_coverages: std::collections::BTreeSet<CoveredLine> = covered_lines
            .difference(&self.covered_lines)
            .cloned()
            .collect();
        if !new_coverages.is_empty() {
            println!("New coverages: {}.", new_coverages.len());
            self.population.push((fuzzied_string.clone(), path_identifier));
            self.covered_lines.append(&mut new_coverages);
        }
        return (fuzzied_string, outcome);
    }

    fn print_results(&self) {
        println!("Total Coverage: {} - Population: {:?}", self.covered_lines.len(), self.population);
    }

}