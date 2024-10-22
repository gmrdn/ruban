// use assert_cmd::prelude::*; // Add methods on commands
// use predicates::prelude::*; // Used for writing assertions
// use std::process::Command; // Run programs

// #[test]
// fn should_add_then_remove_a_task_in_command_line() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("ruban")?;
//     cmd.arg("add");
//     cmd.arg("-t");
//     cmd.arg("Home");
//     cmd.arg("Clean the dishes");
//     cmd.assert()
//         .success()
//         .stdout(predicate::str::contains("Clean the dishes"));

//     cmd = Command::cargo_bin("ruban")?;
//     cmd.arg("rm");
//     cmd.arg("6");
//     cmd.assert()
//         .success()
//         .stdout(predicate::str::contains("Removed task 6"));

//     Ok(())
// }

// #[test]
// fn should_list_all_tasks_in_command_line() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("ruban")?;
//     cmd.arg("ls");
//     cmd.assert()
//         .success()
//         .stdout(predicate::str::contains("All tasks:\n"));

//     Ok(())
// }

// #[test]
// fn should_move_a_task_through_all_status() -> Result<(), Box<dyn std::error::Error>>  {
//     let mut cmd = Command::cargo_bin("ruban")?;
//     cmd.arg("mv");
//     cmd.arg("2");
//     cmd.arg("WIP");
//     cmd.assert()
//         .success()
//         .stdout(predicate::str::contains("Moved task 2 to status WIP"));
//     Ok(())
// }
