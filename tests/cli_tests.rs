use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;

#[test]
fn cli_runs_on_fixture_inputs() {
    // input_1
    let mut expected1 = fs::read_to_string("fixtures/input_1_expected_stdout.txt").unwrap();
    if !expected1.ends_with('\n') {
        expected1.push('\n');
    }
    let mut cmd = Command::cargo_bin("charger-uptime").unwrap();
    cmd.arg("fixtures/input_1.txt");
    cmd.assert().success().stdout(expected1);

    // input_2
    let mut expected2 = fs::read_to_string("fixtures/input_2_expected_stdout.txt").unwrap();
    if !expected2.ends_with('\n') {
        expected2.push('\n');
    }
    let mut cmd2 = Command::cargo_bin("charger-uptime").unwrap();
    cmd2.arg("fixtures/input_2.txt");
    cmd2.assert().success().stdout(expected2);
}
