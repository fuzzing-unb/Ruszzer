use rand::distributions::Uniform;
use rand::Rng;

use super::api::Strategy;

const MAX_STRING_LENGTH: usize = 1000;
const CHAR_START : u8 = 32;
const CHAR_RANGE : u8 = 64;

pub struct RandomStrategy {
    pub max_string_length : usize,
    pub char_start : u8,
    pub char_range : u8,
}

impl RandomStrategy {
    pub fn default() -> RandomStrategy {
        return RandomStrategy {
            max_string_length: MAX_STRING_LENGTH,
            char_start: CHAR_START,
            char_range: CHAR_RANGE, 
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
}
