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
    let trials = 50;
    let fuzzer_runs = fuzzer.runs(&cgi_decode_program_runner, trials);
    
    println!("Random Fuzzer");
    for (i, r) in fuzzer_runs.iter().enumerate() {
        println!("Run #{} coverage {}", i + 1, r.coverage.covered_lines.len());
    }

    // COVERAGE_MUTATION_FUZZER (WIP)
    let mutator = RandomMutator { ..RandomMutator::default() };
    let mutator_runs = mutator.runs(&cgi_decode_program_runner, String::from("A quick brown fox"), trials);

    println!("Mutator Fuzzer");
    for (i, r) in mutator_runs.iter().enumerate() {
        println!("Run #{} coverage {}", i + 1, r.coverage.covered_lines.len());
    }
}
