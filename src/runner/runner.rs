pub trait Runner {
    fn run(&self, args: &String) -> Output;
}

#[derive(Debug)]
pub struct Output {
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String,
}
