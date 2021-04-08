pub trait Runner {
    fn run(&self, args: &String) -> Outcome;
}

// A Covered Line is a Pair of a Function Name (String) and a Line Number
pub type CoveredLine = (String, u64);

#[derive(Debug)]
pub struct CodeCoverage {
    pub covered_lines: std::collections::BTreeSet<CoveredLine>
}

#[derive(Debug)]
pub struct Outcome {
    pub program_outcome: ProgramOutcome,
    pub status_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub coverage: CodeCoverage
}

#[derive(Debug)]
pub enum ProgramOutcome {
    FINISHED,
    HANG,
    SIGNALED
}