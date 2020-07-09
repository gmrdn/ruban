mod rendering;
use crate::rendering::rendering::{confirm_the_tags, confirm_the_task, greet_the_user};
use serde::{Deserialize, Serialize};
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
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    ToDo,
    WIP,
    Done,
}

const STD_OUT_ERR_MSG: &str = "Unable to write message in standard output";

fn main() {
    let source: DataFile = DataFile {
        filepath: "test_tasks.json".to_string(),
    };

    let tasks = retrieve_tasks(source);

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

trait Data {
    fn get_data(&self) -> String;
}

struct DataFile {
    filepath: String,
}

impl Data for DataFile {
    fn get_data(&self) -> String {
        fs::read_to_string(&self.filepath).expect("Something went wrong reading the file")
    }
}

fn retrieve_tasks<T: Data>(source: T) -> Tasks {
    let data_from_file = source.get_data();

    let tasks_from_json: Tasks = serde_json::from_str(data_from_file.as_str())
        .expect("Unable to serialize tasks from Json into struct Tasks");
    tasks_from_json
}

#[test]
fn should_retrieve_tasks() {
    let data = r#"
    {
        "tasks": [
            {
                "number": 1,
                "tags": "Home",
                "task": "Clean the kids room",
                "creation_date": "",
                "status": "ToDo"
            }
        ]
    }"#;
    struct TestData {data: String};
    impl Data for TestData {
        fn get_data(&self) -> String {
            (&self.data).to_string()
        }
    }
    let source = TestData {data: data.to_string() };
    let wanted = "Clean the kids room".to_string();
    let got = retrieve_tasks(source);
    assert_eq!(got.tasks[0].task, wanted);
}
