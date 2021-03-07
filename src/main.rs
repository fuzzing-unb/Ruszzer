mod fuzzer;
mod runner;

use fuzzer::fuzzer::Fuzzer;
use fuzzer::random_fuzzer::RandomFuzzer;
use runner::program_runner::ProgramRunner;
use runner::runner::Runner;

fn main() {
    let fuzzer = RandomFuzzer {};
    let fuzzied_output = fuzzer.fuzz(100, 32, 126);

    let echo_program = ProgramRunner {
        program_name: String::from("echo"),
    };
    let output = echo_program.run(&fuzzied_output);
    println!("{:?}", output)
}
