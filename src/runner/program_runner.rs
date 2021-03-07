use std::process::Command;

use super::runner::{Output, Runner};

pub struct ProgramRunner {
    pub program_name: String,
}

impl Runner for ProgramRunner {
    fn run(&self, args: &String) -> Output {
        let output = Command::new(self.program_name.to_string())
            .arg(args)
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

        return Output {
            status_code: status_code,
            stdout: stdout,
            stderr: stderr,
        };
    }
}
