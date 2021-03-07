use rand::distributions::Uniform;
use rand::Rng;

use super::fuzzer::Fuzzer;

pub struct RandomFuzzer {}

impl Fuzzer for RandomFuzzer {
    fn fuzz(&self, max_length: usize, char_start: u8, char_range: u8) -> String {
        let mut rng = rand::thread_rng();
        let string_size = rng.gen_range(0..max_length + 1);
        let character_distribution = Uniform::from(char_start..char_start + char_range + 1);
        return rng
            .sample_iter(character_distribution)
            .take(string_size)
            .map(char::from)
            .collect();
    }
}
