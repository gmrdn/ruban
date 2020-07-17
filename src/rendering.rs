use crate::taskmanager::{Task, Tasks};

pub fn greet_the_user(mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "Hello, Ruban User.")?;
    Ok(())
}

pub fn confirm_the_task(
    task: &Task,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "{:?}", task)?;
    Ok(())
}

pub fn render_all_tasks(
    tasks: &Tasks,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "All tasks:")?;
    for task in tasks {
        writeln!(writer, "{} - {}", task.number, task.description)?;
    }
    Ok(())
}

#[cfg(test)]
use crate::taskmanager::Status;
#[cfg(test)]
use std::str::from_utf8;

#[test]
fn should_greet_the_user() {
    let mut result = Vec::new();
    greet_the_user(&mut result).expect("");
    assert_eq!(from_utf8(&result).unwrap(), "Hello, Ruban User.\n");
}

#[test]
fn should_confirm_the_task() {
    let task = Task {
        number: 0,
        tags: None,
        description: "Do the laundry".to_string(),
        creation_date: "".to_string(),
        status: Status::ToDo,
    };
    let mut result = Vec::new();
    confirm_the_task(&task, &mut result).expect("");
    assert_eq!(from_utf8(&result).unwrap(), "Task { number: 0, tags: None, description: \"Do the laundry\", creation_date: \"\", status: ToDo }\n");
}

#[test]
fn should_display_all_tasks() {
    let tasks = Tasks {
        tasks: vec![
            Task {
                number: 1,
                tags: Some("House".to_string()),
                description: "Repair the garage door.".to_string(),
                creation_date: "1996-12-19T16:39:57-08:00".to_string(),
                status: Status::ToDo,
            },
            Task {
                number: 2,
                tags: Some("Dev".to_string()),
                description: "Finish the Rust Book.".to_string(),
                creation_date: "1996-12-19T16:39:57-08:00".to_string(),
                status: Status::ToDo,
            },
        ],
    };
    let mut result = Vec::new();

    render_all_tasks(&tasks, &mut result).expect("");
    assert_eq!(
        from_utf8(&result).unwrap(),
        "All tasks:\n1 - Repair the garage door.\n2 - Finish the Rust Book.\n"
    );
}
