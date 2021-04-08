use std::io::Read;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

use super::api::{Outcome, Runner, CodeCoverage, ProgramOutcome};

const TIMEOUT_TOLERANCE_SECONDS: u64 = 1;

pub struct GCovBinaryRunner {
    pub binary_path: String,
    pub binary_name: String,
}

impl Runner for GCovBinaryRunner {
    fn run(&self, args: &String) -> Outcome {
        let mut child = Command::new(format!("./{}", &self.binary_name))
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .arg(args)
            .current_dir(&self.binary_path)
            .spawn()
            .expect("Failed to spawn process.");
        let timeout_tolerance = Duration::from_secs(TIMEOUT_TOLERANCE_SECONDS);
        let (program_outcome, status_code) = match child.wait_timeout(timeout_tolerance).unwrap() {
            // We manually set 139 as the Exit Status code for killed processes (128 + 11 for SIGSEGV)
            // As we consider that the program was terminated by an operating system signal
            Some(status) => (ProgramOutcome::FINISHED, status.code().unwrap_or(139)),
            None => {
                child.kill().ok();
                // We manually set 137 as the Exit Status code for killed processes (128 + 9 for SIGKILL)
                // As it happens that the exit code is None if the program is terminated by a signal
                (ProgramOutcome::HANG, child.wait().unwrap().code().unwrap_or(137))
            }
        };

        let mut stdout = Vec::new();
        child.stdout.unwrap().read_to_end(& mut stdout).unwrap();

        let mut stderr = Vec::new();
        child.stderr.unwrap().read_to_end(&mut stderr).unwrap();

        let source_file_name = format!("{}.c", self.binary_name);
        let gcov_data = extract_gcov_data(&self.binary_path, &source_file_name);
        let coverage = process_gcov_data(&gcov_data, &source_file_name);

        Command::new("rm")
            .arg("-f")
            .arg(format!("{}.gcda", &self.binary_name))
            .current_dir(&self.binary_path)
            .spawn()
            .expect("Failed to spawn process.");

        return Outcome {
            program_outcome,
            status_code,
            stdout,
            stderr,
            coverage
        };
    }
}

fn extract_gcov_data(binary_path: &String, source_file_name: &String) -> String {
    let gcov_output = Command::new("gcov")
        .arg(&source_file_name)
        .arg("--stdout")
        .current_dir(&binary_path)
        .output()
        .expect("Failed to execute gcov process.");
    let gcov_contents = String::from_utf8(gcov_output.stdout)
        .expect("Failed to read the output of the gcov command.");
    return gcov_contents;
}

fn process_gcov_data(gcov_data: &String, source_file_name: &String) -> CodeCoverage {
    let lines = gcov_data.lines();
    let mut covered_lines = std::collections::BTreeSet::new();
    for line in lines {
        let gcov_elements: Vec<&str> = line.split(":").collect();
        let times_covered_str = gcov_elements[0].trim();
        match times_covered_str.parse::<u64>() {
            Ok(times_covered) => {
                if times_covered <= 0 {
                    continue
                }   
            },
            Err(_covered_info_error) => continue
            
        };
        let line_number_str = gcov_elements[1].trim();
        match line_number_str.parse::<u64>() {
            Ok(line_number) => covered_lines.insert((source_file_name.clone(), line_number)),
            Err(_line_number_error) => continue
        };
    }
    return CodeCoverage {
        covered_lines
    }
}