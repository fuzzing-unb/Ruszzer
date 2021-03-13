use rand::distributions::Uniform;
use rand::Rng;

use super::api::Fuzzer;
use crate::runner::api::{Runner, Outcome};

const MAX_STRING_LENGTH: usize = 1000;
const CHAR_START : u8 = 32;
const CHAR_RANGE : u8 = 64;

pub struct RandomFuzzer {
    pub max_string_length : usize,
    pub char_start : u8,
    pub char_range : u8,
}

impl RandomFuzzer {
    pub fn default() -> RandomFuzzer {
        return RandomFuzzer {
            max_string_length: MAX_STRING_LENGTH,
            char_start: CHAR_START,
            char_range: CHAR_RANGE, 
        }
    }
}

impl Fuzzer for RandomFuzzer {

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

    fn run(&self, runner: &dyn Runner) -> Outcome {
        let fuzzied_string = self.fuzz();
        return runner.run(&fuzzied_string);
    }

    fn runs(&self, runner: &dyn Runner, trials: usize) -> Vec<Outcome> {
        let mut vec = Vec::with_capacity(trials);
        for _trial in 1..=trials {
            vec.push(self.run(runner));
        }
        return vec;
    }

}
