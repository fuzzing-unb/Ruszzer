mod fuzzer;
mod runner;

use fuzzer::random_fuzzer::RandomFuzzer;
use fuzzer::api::Fuzzer;
use runner::gcov_binary_runner::GCovBinaryRunner;

fn main() {
    let fuzzer = RandomFuzzer { ..RandomFuzzer::default() };

    let echo_program_runner = GCovBinaryRunner {
        binary_path: String::from("fuzzy_targets"),
        binary_name: String::from("cgi_decode"),
    };
    let trials = 1;
    let output = fuzzer.runs(&echo_program_runner, trials);
    println!("{:?}", output)
}
