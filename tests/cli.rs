use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn add_a_task() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("ruban")?;
    cmd.arg("étendre le linge");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("étendre le linge"));

    Ok(())
}