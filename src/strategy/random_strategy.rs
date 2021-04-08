use rand::distributions::Uniform;
use rand::Rng;
use crate::runner::api::{Runner, Outcome, CoveredLine};

use super::api::Strategy;

const MAX_STRING_LENGTH: usize = 1000;
const CHAR_START : u8 = 32;
const CHAR_RANGE : u8 = 64;

pub struct RandomStrategy<'a> {
    pub runner: &'a dyn Runner,
    pub max_string_length : usize,
    pub char_start : u8,
    pub char_range : u8,
    pub covered_lines: std::collections::BTreeSet<CoveredLine>,
}

impl <'a> RandomStrategy<'a> {
    pub fn default(runner: &'a dyn Runner) -> RandomStrategy<'a> {
        return RandomStrategy {
            runner,
            max_string_length: MAX_STRING_LENGTH,
            char_start: CHAR_START,
            char_range: CHAR_RANGE,
            covered_lines: std::collections::BTreeSet::new()
        }
    }
} 

impl <'a> Strategy for RandomStrategy<'a> {

    fn fuzz(&mut self) -> String {
        let mut rng = rand::thread_rng();
        let string_size = rng.gen_range(0..self.max_string_length + 1);
        let character_distribution = Uniform::from(self.char_start..=self.char_start + self.char_range);
        return rng
            .sample_iter(character_distribution)
            .take(string_size)
            .map(char::from)
            .collect();
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
        }
        self.covered_lines.append(&mut new_coverages);
        return (fuzzied_string, outcome);
    }

    fn print_results(&self) {
        println!("Total Coverage: {}", self.covered_lines.len())
    }

}
