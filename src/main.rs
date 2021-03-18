mod fuzzer;
mod runner;
mod mutator;

use fuzzer::random_fuzzer::RandomFuzzer;
use fuzzer::api::Fuzzer;
use runner::gcov_binary_runner::GCovBinaryRunner;
use mutator::random_mutator::RandomMutator;
use mutator::api::Mutator;

fn main() {

    let cgi_decode_program_runner = GCovBinaryRunner {
        binary_path: String::from("fuzzy_targets"),
        binary_name: String::from("cgi_decode"),
    };

    // RANDOM_FUZZER
    let fuzzer = RandomFuzzer { ..RandomFuzzer::default() };
    let trials = 100;
    fuzzer.runs(&cgi_decode_program_runner, trials);

    // COVERAGE_MUTATION_FUZZER (WIP)
    let mutator = RandomMutator {
        min_mutations: 1,
        max_mutations: 1
    };
    println!("{}", mutator.mutate(&String::from("cebola")));
}
