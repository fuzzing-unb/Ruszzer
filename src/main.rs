mod runner;
mod strategy;
mod mutator;

use runner::gcov_binary_runner::GCovBinaryRunner;
use strategy::api::Strategy;
use strategy::mutation_strategy::MutationStrategy;
use strategy::random_strategy::RandomStrategy;
use strategy::greybox_strategy::GreyboxStrategy;
use strategy::boosted_greybox_strategy::BoostedGreyboxStrategy;
use mutator::random_mutator::RandomMutator;

fn main() {
    // Prepare option when have a CLI.
    let strategy_option = "boosted_greybox";
    let seed = String::from("http://www.google.com/search?q=fuzzing");
    let trials = 1000;
    let runner = GCovBinaryRunner {
        binary_path: String::from("fuzzy_targets"),
        binary_name: String::from("cgi_decode"),
    };
    let mutator = RandomMutator::default();

    let mut strategy : Box<dyn Strategy> = match strategy_option {
        "mutation" => {
            Box::new(MutationStrategy{ seed, ..MutationStrategy::default(&mutator, &runner) })
        },
        "random" => { 
            Box::new(RandomStrategy{ ..RandomStrategy::default(&runner) })
        },
        "greybox" => { 
            Box::new(GreyboxStrategy{ seed, ..GreyboxStrategy::default(&mutator, &runner) })
        },
        "boosted_greybox" => {
            Box::new(BoostedGreyboxStrategy{ seed, ..BoostedGreyboxStrategy::default(&mutator, &runner) })
        }
        _ => panic!(),
    };
    strategy.runs(trials);
}
