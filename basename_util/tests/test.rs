use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};

#[test]
fn prints_basename_test() {
    let _output = Command::cargo_bin("basename_util")
        .unwrap()
        .arg("/usr/bin/sort")
        .assert()
        .stdout("sort\n");
}

#[test]
fn multiple_inputs_test() {
    let _output = Command::cargo_bin("basename_util")
        .unwrap()
        .args(["-a", "/usr/bin/sort", "/usr/bin/test"])
        .assert()
        .stdout("sort\ntest\n");
}

#[test]
fn extension_test() {
    let _output = Command::cargo_bin("basename_util")
        .unwrap()
        .args(["-s", ".h", "/usr/kol/lol.h"])
        .assert()
        .stdout("lol\n");
}
#[test]
fn plain_test() {
    let _output = Command::cargo_bin("basename_util")
        .unwrap()
        .args(["-s", "/usr/kol/lol.h"])
        .assert()
        .stdout("lol.h\n");
}

#[test]
fn no_newline_test() {
    let _output = Command::cargo_bin("basename_util")
        .unwrap()
        .args(["-z", "/usr/kol/bin"])
        .assert()
        .stdout("bin");
}
