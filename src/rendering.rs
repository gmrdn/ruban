use crate::taskmanager::{Status, Task, Tasks};
use prettytable::{color, Attr, Cell, Row, Table};

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

pub fn confirm_task_removed(
    number: u32,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "Removed task {}", number)?;
    Ok(())
}

pub fn confirm_task_moved(
    number: u32,
    status: &str,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "Moved task {} to status {}", number, status)?;
    Ok(())
}

pub fn render_all_tasks(
    tasks: &Tasks,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "All tasks:")?;

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("To Do")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Work in Progress")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Done")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));

    let mut tasks_by_statuses: Vec<Vec<Task>> = vec![];

    for status in Status::iterator() {
        tasks_by_statuses.push(tasks.tasks_by_status(status))
    }

    for i in 0..tasks.into_iter().len() {
        let mut cells = vec![];

        for tasks_by_status in &tasks_by_statuses {
            let tasks_current_status_and_current_row = tasks_by_status.get(i);

            match tasks_current_status_and_current_row {
                Some(t) => cells.push(Cell::new(
                    textwrap::fill(format!("{} - {}", t.number, t.description).as_str(), 24)
                        .as_str(),
                )),
                None => cells.push(Cell::new("")),
            };
        }
        for cell in &cells {
            if cell.get_content() != "" {
                table.add_row(Row::from(cells));
                break;
            }
        }
    }

    table
        .print(&mut writer)
        .expect("Unable to write table in writer");
    Ok(())
}

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
            Task {
                number: 3,
                tags: Some("House".to_string()),
                description: "Pay the bills".to_string(),
                creation_date: "1996-12-19T16:39:57-08:00".to_string(),
                status: Status::Done,
            },
            Task {
                number: 4,
                tags: Some("Dev".to_string()),
                description: "Write unit tests".to_string(),
                creation_date: "1996-12-19T16:39:57-08:00".to_string(),
                status: Status::WIP,
            },
            Task {
                number: 5,
                tags: Some("Dev".to_string()),
                description: "Read the doc".to_string(),
                creation_date: "1996-12-19T16:39:57-08:00".to_string(),
                status: Status::Done,
            },
        ],
    };
    let mut result = Vec::new();

    render_all_tasks(&tasks, &mut result).expect("unable to render all tasks");
    assert_eq!(
        from_utf8(&result).unwrap(),
        "All tasks:
+-----------------------+----------------------+-------------------+
| To Do                 | Work in Progress     | Done              |
+-----------------------+----------------------+-------------------+
| 1 - Repair the garage | 4 - Write unit tests | 3 - Pay the bills |
| door.                 |                      |                   |
+-----------------------+----------------------+-------------------+
| 2 - Finish the Rust   |                      | 5 - Read the doc  |
| Book.                 |                      |                   |
+-----------------------+----------------------+-------------------+
"
    );
}
