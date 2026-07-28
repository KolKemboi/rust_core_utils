use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};

#[test]
fn process_slash_test() {
    let _path = Command::cargo_bin("dirname_util")
        .unwrap()
        .arg("/")
        .assert()
        .stdout("/\n");
}

#[test]
fn process_dot_test() {
    let _path = Command::cargo_bin("dirname_util")
        .unwrap()
        .arg("stdio.h")
        .assert()
        .stdout(".\n");
}

#[test]
fn process_z_test() {
    let _path = Command::cargo_bin("dirname_util")
        .unwrap()
        .args(["-z", "home/kol/bin"])
        .assert()
        .stdout("home/kol");
}

#[test]
fn process_multiple_test() {
    let _path = Command::cargo_bin("dirname_util")
        .unwrap()
        .args(["dir_1/kol", "dir_2/kol"])
        .assert()
        .stdout("dir_1\ndir_2\n");
}
