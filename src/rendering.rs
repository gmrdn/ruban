use crate::taskmanager::{Status, Task, Tasks};
use prettytable::Table;

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
    table.add_row(row!["To Do", "WIP", "Done"]);

    let tasks_todo = tasks
        .into_iter()
        .filter(|t| t.status == Status::ToDo)
        .cloned()
        .collect::<Vec<Task>>();
    let tasks_wip = tasks
        .into_iter()
        .filter(|t| t.status == Status::WIP)
        .cloned()
        .collect::<Vec<Task>>();
    let tasks_done = tasks
        .into_iter()
        .filter(|t| t.status == Status::Done)
        .cloned()
        .collect::<Vec<Task>>();

    for i in 0..tasks.into_iter().len() {
        let todo: &str;
        let wip: &str;
        let done: &str;

        match tasks_todo.iter().nth(i) {
            Some(t) => todo = t.description.as_str(),
            None => todo = "",
        };

        match tasks_wip.iter().nth(i) {
            Some(t) => wip = t.description.as_str(),
            None => wip = "",
        };

        match tasks_done.iter().nth(i) {
            Some(t) => done = t.description.as_str(),
            None => done = "",
        };

        if todo != "" || wip != "" || done != "" {
            table.add_row(row![todo, wip, done]);
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
                number: 4,
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
+-------------------------+------------------+---------------+
| To Do                   | WIP              | Done          |
+-------------------------+------------------+---------------+
| Repair the garage door. | Write unit tests | Pay the bills |
+-------------------------+------------------+---------------+
| Finish the Rust Book.   |                  | Read the doc  |
+-------------------------+------------------+---------------+
"
    );
}
