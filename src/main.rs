mod fuzzer;
mod runner;
mod strategy;

use clap::{App, load_yaml};
use fuzzer::fuzzer_impl::FuzzerImpl;
use fuzzer::api::Fuzzer;
use runner::gcov_binary_runner::GCovBinaryRunner;
use strategy::mutation_strategy::MutationStrategy;
use strategy::random_strategy::RandomStrategy;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let strategy_option = matches.value_of("strategy").unwrap();
    let input_path = matches.value_of("INPUT").unwrap();

    let split = input_path.split("/");
    let path_vector = split.collect::<Vec<&str>>();
    let binary_path = path_vector[0];
    let binary_name = path_vector[1];


    let runner = GCovBinaryRunner {
        binary_path: String::from(binary_path),
        binary_name: String::from(binary_name),
    };

    println!("Running {} fuzzer", strategy_option);

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
