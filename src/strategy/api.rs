
use crate::runner::api::{Outcome, ProgramOutcome};

pub trait Strategy {

    fn fuzz(&mut self) -> String;

    fn run(&mut self) -> (String, Outcome);

    fn runs(&mut self, trials: usize) -> Vec<(String, Outcome)> {
        let mut vec = Vec::with_capacity(trials);
        for _trial in 1..=trials {
            let outcome = self.run();
            if matches!(outcome.1.program_outcome, ProgramOutcome::HANG) || matches!(outcome.1.program_outcome, ProgramOutcome::SIGNALED) {
                println!("Outcome {:?} with code {} for String '{}'", outcome.1.program_outcome, outcome.1.status_code, outcome.0);
            }
            vec.push(outcome);
        }
        return vec;
    }

    fn print_results(&self);

}
