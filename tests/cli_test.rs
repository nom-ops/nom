use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_typo_records_mistake() {
    let mut cmd = Command::cargo_bin("nom").unwrap();
    cmd.arg("smce").assert().failure().stdout(predicate::str::contains("Mistake #"));
}

#[test]
fn test_valid_command_passes_through() {
    let mut cmd = Command::cargo_bin("nom").unwrap();
    cmd.arg("--version").assert().success();
}
