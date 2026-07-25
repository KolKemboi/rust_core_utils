use assert_cmd::Command;

#[test]
fn exit_unsuccessfully() {
    Command::cargo_bin("false_util") // looks for the false binary
        .unwrap() // panics when it doesn't find it
        .assert() // runs the binary and captures the exit code
        .failure(); // checks if the exit code is non zero
}
