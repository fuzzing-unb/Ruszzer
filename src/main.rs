mod fuzzer;
mod runner;
mod strategy;

use fuzzer::random_fuzzer::RandomFuzzer;
use crate::runner::api::Outcome;
use fuzzer::api::Fuzzer;
use runner::gcov_binary_runner::GCovBinaryRunner;
use strategy::mutation_strategy::MutationStrategy;
use strategy::random_strategy::RandomStrategy;

fn main() {
    let strategy_option = "mutation";

    let cgi_decode_program_runner = GCovBinaryRunner {
        binary_path: String::from("fuzzy_targets"),
        binary_name: String::from("cgi_decode"),
    };

    // RANDOM_FUZZER
    let trials = 50;
    let fuzzer = RandomFuzzer { ..RandomFuzzer::default() };

    let fuzzer_runs: Vec<Outcome>;
    match strategy_option {
        "mutation" => {
            let strategy = MutationStrategy { ..MutationStrategy::default()};
            fuzzer_runs = fuzzer.runs(&cgi_decode_program_runner, &strategy, trials);
        },
        _ => {
            let strategy = RandomStrategy { ..RandomStrategy::default()};
            fuzzer_runs = fuzzer.runs(&cgi_decode_program_runner, &strategy, trials);
        }
    }
    
    println!("Random Fuzzer");
    for (i, r) in fuzzer_runs.iter().enumerate() {
        println!("Run #{} coverage {}", i + 1, r.coverage.covered_lines.len());
    }
}
