mod runner;
mod strategy;
mod mutator;

use clap::{App, load_yaml};
use runner::gcov_binary_runner::GCovBinaryRunner;
use strategy::api::Strategy;
use strategy::mutation_strategy::MutationStrategy;
use strategy::random_strategy::RandomStrategy;
use strategy::greybox_strategy::GreyboxStrategy;
use strategy::boosted_greybox_strategy::BoostedGreyboxStrategy;
use mutator::random_mutator::RandomMutator;

fn main() {

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let strategy_option = matches.value_of("fuzzer").unwrap();
    let seeds : Vec<String> = matches.values_of("seeds").unwrap().map(String::from).collect();
    let trials = matches.value_of("trials").unwrap().parse().unwrap();
    let input_path = matches.value_of("input").unwrap();
    let split = input_path.split("/");
    let path_vector = split.collect::<Vec<&str>>();
    let binary_path = path_vector[0];
    let binary_name = path_vector[1];

    let runner = GCovBinaryRunner {
        binary_path: String::from(binary_path),
        binary_name: String::from(binary_name),
    };
    let mutator = RandomMutator::default();

    let mut strategy : Box<dyn Strategy> = match strategy_option {
        "mutation" => {
            Box::new(MutationStrategy{ seeds, ..MutationStrategy::default(&mutator, &runner) })
        },
        "random" => { 
            Box::new(RandomStrategy{ ..RandomStrategy::default(&runner) })
        },
        "greybox" => { 
            Box::new(GreyboxStrategy{ seeds, ..GreyboxStrategy::default(&mutator, &runner) })
        },
        "boosted_greybox" => {
            Box::new(BoostedGreyboxStrategy{ seeds, ..BoostedGreyboxStrategy::default(&mutator, &runner) })
        }
        _ => panic!(),
    };
    let outcomes = strategy.runs(trials);
    for outcome in outcomes {
        println!("{} {:?} {}", outcome.0, outcome.1.program_outcome, outcome.1.status_code);
    }
    strategy.print_results();
}
