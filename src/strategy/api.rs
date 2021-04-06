
use crate::runner::api::Outcome;

pub trait Strategy {

    fn fuzz(&self) -> String;

    fn run(&mut self) -> (String, Outcome);

    fn runs(&mut self, trials: usize) -> Vec<(String, Outcome)> {
        let mut vec = Vec::with_capacity(trials);
        for _trial in 1..=trials {
            vec.push(self.run());
        }
        return vec;
    }

    fn print_results(&self);

}
