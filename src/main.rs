use std::str::from_utf8;
use structopt::StructOpt;
use std::error::Error;

#[derive(StructOpt)]
enum Ruban {
    Add {
        #[structopt(default_value = "", short = "t", long = "tags")]
        tags: String,
        task: String,
    },
    Ls {},
    Rm {},
    Mv {},
}

#[derive(Debug)]
struct Tasks {
    tasks: Vec<Task>,
}

impl IntoIterator for Tasks {
    type Item = Task;
    type IntoIter = ::std::vec::IntoIter<Task>;

    fn into_iter(self) -> Self::IntoIter {
        self.tasks.into_iter()
    }
}

impl<'s> IntoIterator for &'s Tasks {
    type Item = &'s Task;
    type IntoIter = ::std::slice::Iter<'s, Task>;

    fn into_iter(self) -> Self::IntoIter {
        self.tasks.iter()
    }
}

#[derive(Debug)]
struct Task {
    number: u32,
    tags: Option<String>,
    task: String,
    creation_date: Option<String>,
    status: Option<Status>,
}

#[derive(Debug)]
enum Status {}

fn main() -> Result<(), Box<dyn Error>> {
    greet_the_user(&mut std::io::stdout());
    match Ruban::from_args() {
        Ruban::Add { task, tags} => {
            confirm_the_task(task, &mut std::io::stdout());
            confirm_the_tags(tags, &mut std::io::stdout())
        }
        _ => Ok(()),
    }
}

fn greet_the_user(mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "Hello, Ruban User.")?;
    Ok(())
}

fn confirm_the_task(
    task: String,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "New task: {}", task)?;
    Ok(())
}

fn confirm_the_tags(
    tags: String,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "Tags: {}", tags)?;
    Ok(())
}

fn render_all_tasks(
    tasks: &Tasks,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    for task in tasks {
        writeln!(writer, "{} - {}", task.number, task.task)?;
    }
    Ok(())
}

#[test]
fn should_greet_the_user() {
    let mut result = Vec::new();
    greet_the_user(&mut result);
    assert_eq!(from_utf8(&result).unwrap(), "Hello, Ruban User.\n");
}

#[test]
fn should_confirm_the_task() {
    let task = String::from("Do the laundry");
    let mut result = Vec::new();
    confirm_the_task(task, &mut result);
    assert_eq!(from_utf8(&result).unwrap(), "New task: Do the laundry\n");
}

#[test]
fn should_confirm_the_tags() {
    let tags = String::from("House");
    let mut result = Vec::new();
    confirm_the_tags(tags, &mut result);
    assert_eq!(from_utf8(&result).unwrap(), "Tags: House\n");
}

#[test]
fn should_display_all_tasks() {
    let tasks = Tasks {
        tasks: vec![
            Task {
                number: 1,
                tags: Some("House".to_string()),
                task: "Repair the garage door.".to_string(),
                creation_date: None,
                status: None,
            },
            Task {
                number: 2,
                tags: Some("Dev".to_string()),
                task: "Finish the Rust Book.".to_string(),
                creation_date: None,
                status: None,
            },
        ],
    };
    let mut result = Vec::new();

    render_all_tasks(&tasks, &mut result);
    assert_eq!(
        from_utf8(&result).unwrap(),
        "1 - Repair the garage door.\n2 - Finish the Rust Book.\n"
    );
}
