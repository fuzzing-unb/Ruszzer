mod fuzzer;
mod runner;

use fuzzer::fuzzer::Fuzzer;
use fuzzer::random_fuzzer::RandomFuzzer;
use runner::program_runner::run_program;

fn main() {
    let fuzzer = RandomFuzzer {};
    let fuzzied_output = fuzzer.fuzz(100, 32, 126);

    run_program(&String::from("echo"), &fuzzied_output);
}
