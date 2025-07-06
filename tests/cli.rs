use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn passes_with_valid_fixture() {
    Command::cargo_bin("aascheck")
        .unwrap()
        .args([
            "--mode",
            "AAS",
            "tests/Example-Simple.json",
        ])
        // Expect exit-status 0
        .assert()
        .success()
        // And no diagnostic output on stderr
        .stderr(predicate::str::is_empty());
}

#[test]
fn fails_with_invalid_fixture() {
    Command::cargo_bin("aascheck")
        .unwrap()
        .args([
            "--mode",
            "AAS",
            "tests/invalid.json",
        ])
        // Expect exit-status 0
        .assert()
        .failure();
}