use std::process::Command;

pub fn run_program(program_name: &String, args: &String) {
    let output = Command::new(program_name)
        .arg(args)
        .output()
        .expect("Failed to execute process.")
        .stdout;
    let output = std::str::from_utf8(&output).unwrap();
    println!("{}", output);
}
