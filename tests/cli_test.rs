use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-linter")?;

    cmd.arg("");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unable to open file"));

    Ok(())
}

// Integration test related with fixtures
#[test]
fn file_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-linter")?;

    cmd.arg("tests/fixtures/test.txt");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Error in line 6, extra comma"));

    Ok(())
}
