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
        let source_file_name = format!("{}.c", self.binary_name);
        let gcov_data = extract_gcov_data(&self.binary_path, &source_file_name);
        let coverage = process_gcov_data(&gcov_data, &source_file_name);

        return Outcome {
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