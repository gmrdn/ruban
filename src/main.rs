mod adapter_dataprovider;
mod port_dataprovider;
mod rendering;
mod types;

use crate::adapter_dataprovider::DataFile;
use crate::port_dataprovider::DataProvider;
use crate::rendering::rendering::{
    confirm_the_tags, confirm_the_task, greet_the_user, render_all_tasks,
};
use crate::types::{Cli, Tasks};
use serde::{Deserialize, Serialize};
use std::fs;
use structopt::StructOpt;

const STD_OUT_ERR_MSG: &str = "Unable to write message in standard output";

fn main() {
    let source: DataFile = DataFile {
        filepath: "test_tasks.json".to_string(),
    };

    let tasks = retrieve_tasks(source);

    greet_the_user(&mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
    match Cli::from_args() {
        Cli::Add { task, tags } => {
            confirm_the_task(task, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
            confirm_the_tags(tags, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
        }
        Cli::Ls {} => {
            render_all_tasks(&tasks, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
        }
        _ => (),
    }
}

fn retrieve_tasks<T: DataProvider>(source: T) -> Tasks {
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
    struct TestData {
        data: String,
    };
    impl DataProvider for TestData {
        fn get_data(&self) -> String {
            (&self.data).to_string()
        }
    }
    let source = TestData {
        data: data.to_string(),
    };
    let wanted = "Clean the kids room".to_string();
    let got = retrieve_tasks(source);
    assert_eq!(got.tasks[0].task, wanted);
}
