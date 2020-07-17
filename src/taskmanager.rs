use crate::port_dataprovider::DataProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub number: u32,
    pub tags: Option<String>,
    pub description: String,
    pub creation_date: String,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    ToDo,
    WIP,
    Done,
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

impl Tasks {
    pub fn from(source: impl DataProvider) -> Tasks {
        serde_json::from_str(source.get_data().as_str())
            .expect("Unable to serialize tasks from Json into struct Tasks")
    }

    pub fn add(&mut self, task: &Task,
               mut writer: impl std::io::Write,
    ) {
        self.tasks.push(task.clone());
        serde_json::to_writer_pretty(writer, &task).expect("Unable to write task to file");
    }

    pub fn add_task_from_json(
        &mut self,
        json_task: String,
        mut writer: impl std::io::Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let task: Task = serde_json::from_str(json_task.as_str())
            .expect("Unable to serialize task from Json into struct Task");
        self.tasks.push(task.clone());
        serde_json::to_writer_pretty(writer, &task).expect("Unable to write task to file");
        Ok(())
    }
}

#[cfg(test)]
use std::str::from_utf8;

#[test]
fn should_serialize_from_source() {
    let data = r#"
    {
        "tasks": [
            {
                "number": 1,
                "tags": "Home",
                "description": "Clean the kids room",
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
    let got = Tasks::from(source);
    assert_eq!(got.tasks[0].description, wanted);
}

#[test]
fn should_create_tasks() {
    let mut tasks: Tasks = Tasks { tasks: vec![] };
    let mut result = Vec::new();
    let data = r#"{
  "number": 1,
  "tags": "Home",
  "description": "Clean the kids room",
  "creation_date": "",
  "status": "ToDo"
}"#;

    assert!(tasks
        .add_task_from_json(data.to_string(), &mut result)
        .is_ok());
    assert_eq!(tasks.tasks.len(), 1);
    assert_eq!(tasks.tasks[0].description, "Clean the kids room");
    assert_eq!(from_utf8(&result).unwrap(), data);
}
