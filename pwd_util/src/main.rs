use std::env;
use std::process::Command;
use std::process::ExitCode;

fn main() -> ExitCode {
    let path = env::current_dir().unwrap();

    let status = Command::new("echo_util")
        .arg(path.display().to_string())
        .status()
        .unwrap();

    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
