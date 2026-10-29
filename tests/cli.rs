use std::process::{Command, Output};

fn run_cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rust-cli-template"))
        .args(args)
        .output()
        .expect("failed to run CLI")
}

#[test]
fn hello_command_greets_the_given_name() {
    let output = run_cli(&["hello", "Ada"]);

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, Ada!\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn hello_command_help_describes_the_name_argument() {
    let output = run_cli(&["hello", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Print a greeting"));
    assert!(stdout.contains("<NAME>"));
}

#[test]
fn hello_command_requires_a_name() {
    let output = run_cli(&["hello"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("Usage:"));
}
