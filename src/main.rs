mod rendering;
use crate::rendering::rendering::{confirm_the_tags, confirm_the_task, greet_the_user};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use structopt::StructOpt;

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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    number: u32,
    tags: Option<String>,
    task: String,
    creation_date: Option<String>,
    status: Option<Status>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {}

const STD_OUT_ERR_MSG: &str = "Unable to write message in standard output";

fn main() {
    let tasks = retrieve_tasks();

    greet_the_user(&mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
    match Ruban::from_args() {
        Ruban::Add { task, tags } => {
            confirm_the_task(task, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
            confirm_the_tags(tags, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
        }
        Ruban::Ls {} => {
            rendering::rendering::render_all_tasks(&tasks, &mut std::io::stdout())
                .expect(STD_OUT_ERR_MSG);
        }
        _ => (),
    }
}

fn retrieve_tasks() -> Tasks {
    let filename = "test_tasks.json";
    let data_from_file =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    let tasks_from_json: Tasks = serde_json::from_str(data_from_file.as_str())
        .expect("Unable to serialize tasks from Json into struct Tasks");
    tasks_from_json
}
