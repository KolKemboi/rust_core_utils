use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn no_input_test() {
    Command::cargo_bin("echo_util") // j
        .unwrap() //
        .assert() //
        .success() //
        .stdout("\n"); //
}

#[test]
fn one_input_test() {
    Command::cargo_bin("echo_util") //
        .unwrap() //
        .arg("hello") //
        .assert() //
        .success() //
        .stdout("hello\n"); //
}

#[test]
fn multiple_input_test() {
    Command::cargo_bin("echo_util")
        .unwrap()
        .args(["hello", "world"])
        .assert()
        .success()
        .stdout("hello world\n");
}
