pub trait Mutator {
    fn mutate(&self, target: &String) -> String;
}
