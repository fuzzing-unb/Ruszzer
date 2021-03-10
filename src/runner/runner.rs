pub trait Runner {
    fn run(&self, args: &String) -> Outcome;
}

#[derive(Debug)]
pub struct Outcome {
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String,
}
