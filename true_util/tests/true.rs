use assert_cmd::Command;

#[test]
fn exits_succesfully() {
    Command::cargo_bin("true_util") //locates the binary by name
        .unwrap() // panics if bin not found
        .assert() // this executes the binary and captures the exit status
        .success(); // this checks for the exit code, this should be 0
}
