mod fuzzer;

use fuzzer::random_fuzzer::RandomFuzzer;
use fuzzer::fuzzer::Fuzzer;

fn main() {
    let fuzzer = RandomFuzzer {};
    let fuzzied_output = fuzzer.fuzz(100, 32, 126);
    println!("{}", fuzzied_output);
}