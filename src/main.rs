mod fuzzer;

use fuzzer::fuzzer::Fuzzer;
use fuzzer::random_fuzzer::RandomFuzzer;

fn main() {
    let fuzzer = RandomFuzzer {};
    let fuzzied_output = fuzzer.fuzz(100, 32, 126);
    println!("{}", fuzzied_output);
}
