use rand::distributions::Uniform;
use rand::Rng;
use crate::runner::api::{Runner, Outcome, CoveredLine};

use super::api::Strategy;

const MAX_STRING_LENGTH: usize = 1000;
const CHAR_START : u8 = 32;
const CHAR_RANGE : u8 = 64;

pub struct RandomStrategy {
    pub max_string_length : usize,
    pub char_start : u8,
    pub char_range : u8,
    pub covered_lines: std::collections::BTreeSet<CoveredLine>,
}

impl RandomStrategy {
    pub fn default() -> RandomStrategy {
        return RandomStrategy {
            max_string_length: MAX_STRING_LENGTH,
            char_start: CHAR_START,
            char_range: CHAR_RANGE,
            covered_lines: std::collections::BTreeSet::new()
        }
    }
} 

impl Strategy for RandomStrategy {
    fn fuzz(&self) -> String {
        let mut rng = rand::thread_rng();
        let string_size = rng.gen_range(0..self.max_string_length + 1);
        let character_distribution = Uniform::from(self.char_start..self.char_start + self.char_range + 1);
        return rng
            .sample_iter(character_distribution)
            .take(string_size)
            .map(char::from)
            .collect();
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
