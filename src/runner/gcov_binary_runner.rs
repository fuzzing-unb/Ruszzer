use std::process::Command;

use super::api::{Outcome, Runner, CodeCoverage};

pub struct GCovBinaryRunner {
    pub binary_path: String,
    pub binary_name: String,
}

impl Runner for GCovBinaryRunner {
    fn run(&self, args: &String) -> Outcome {
        let output = Command::new(format!("./{}",&self.binary_name))
            .arg(args)
            .current_dir(&self.binary_path)
            .output()
            .expect("Failed to execute process.");
        let status_code = output
            .status
            .code()
            .expect("Failed to process the status code of the program.");
        let stdout = String::from_utf8(output.stdout)
            .expect("Failed to process the stdout result of the program.");
        let stderr = String::from_utf8(output.stderr)
            .expect("Failed to process the stderr result of the program.");
        let coverage = extract_coverage_information(&self.binary_path, &self.binary_name);

        return Outcome {
            status_code,
            stdout,
            stderr,
            coverage
        };
    }
}

fn extract_coverage_information(binary_path: &String, binary_name: &String) -> CodeCoverage {
    let gcov_output = Command::new("gcov")
        .arg(&binary_name)
        .arg("--stdout")
        .current_dir(&binary_path)
        .output()
        .expect("Failed to execute gcov process.");
    let gcov_contents = String::from_utf8(gcov_output.stdout).expect("Failed to read the output of the gcov command.");
    println!("{}", gcov_contents);
    return CodeCoverage {
        covered_lines: std::collections::BTreeSet::new()
    }
}