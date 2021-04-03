mod fuzzer;
mod runner;
mod strategy;

use fuzzer::fuzzer_impl::FuzzerImpl;
use fuzzer::api::Fuzzer;
use runner::gcov_binary_runner::GCovBinaryRunner;
use strategy::mutation_strategy::MutationStrategy;
use strategy::random_strategy::RandomStrategy;

fn main() {
    // Prepare option when have a CLI.
    let strategy_option = "random";

    let runner = GCovBinaryRunner {
        binary_path: String::from("fuzzy_targets"),
        binary_name: String::from("cgi_decode"),
    };

    // RANDOM_FUZZER
    let trials = 1000;
    let mut fuzzer = FuzzerImpl { ..FuzzerImpl::default() };

    // Run fuzzer according to the chosen strategy
    match strategy_option {
        "mutation" => fuzzer.runs(&runner, &MutationStrategy{ ..MutationStrategy::default() }, trials),
        "random"   => fuzzer.runs(&runner, &RandomStrategy{ ..RandomStrategy::default() }, trials),
        _          => panic!(),
    };
    
    println!("Total coverage: {}", fuzzer.covered_lines.len());
}
