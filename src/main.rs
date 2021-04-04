mod runner;
mod strategy;
mod mutator;

use runner::gcov_binary_runner::GCovBinaryRunner;
use strategy::api::Strategy;
use strategy::mutation_strategy::MutationStrategy;
use strategy::random_strategy::RandomStrategy;
use strategy::greybox_strategy::GreyboxStrategy;
use mutator::random_mutator::RandomMutator;

fn main() {
    // Prepare option when have a CLI.
    let strategy_option = "greybox";
    let seed = String::from("http://www.google.com/search?q=fuzzing");
    let trials = 1000;
    let runner = GCovBinaryRunner {
        binary_path: String::from("fuzzy_targets"),
        binary_name: String::from("cgi_decode"),
    };
    let mutator = RandomMutator::default();

    // Run fuzzer according to the chosen strategy
    // TODO: refine this to use a trait object
    // https://stackoverflow.com/questions/27567849/what-makes-something-a-trait-object
    let covered_lines = match strategy_option {
        "mutation" => {
            let mut strategy = MutationStrategy{ ..MutationStrategy::default(&mutator)};
            strategy.runs(&runner, trials);
            strategy.covered_lines
        },
        "random" => {
            let mut strategy =  RandomStrategy{ ..RandomStrategy::default() };
            strategy.runs(&runner, trials);
            strategy.covered_lines
        },
        "greybox" => {
            let mut strategy = GreyboxStrategy{ seed, ..GreyboxStrategy::default(&mutator) };
            strategy.runs(&runner, trials);
            for element in strategy.population {
                println!("{}", element);
            }
            strategy.covered_lines
        }
        _ => panic!(),
    };
    
    println!("Total coverage: {}", covered_lines.len());
}
