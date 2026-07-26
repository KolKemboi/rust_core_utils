use assert_cmd::Command;
use predicates::str::is_empty;
use std::env;

#[test]
fn exits_successfully() {
    Command::cargo_bin("pwd_util").unwrap().assert().success();
}

#[test]
fn prints_current_directory() {
    let expected = env::current_dir().unwrap();

    Command::cargo_bin("pwd_util")
        .unwrap()
        .assert()
        .success()
        .stdout(format!("{}\n", expected.display()));
}

#[test]
fn stderr_is_empty() {
    Command::cargo_bin("pwd_util")
        .unwrap()
        .assert()
        .stderr(is_empty());
}
