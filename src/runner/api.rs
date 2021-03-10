pub trait Runner {
    fn run(&self, args: &String) -> Outcome;
}

// A Covered Line is a Pair of a Function Name (String) and a Line Number
type CoveredLine = (String, u64);

#[derive(Debug)]
pub struct CodeCoverage {
    pub covered_lines: std::collections::BTreeSet<CoveredLine>
}

#[derive(Debug)]
pub struct Outcome {
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub coverage: CodeCoverage
}
