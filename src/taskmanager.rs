use serde::{Deserialize, Serialize};
use std::io::Read;

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
    pub fn from(reader: impl Read) -> Tasks {
        serde_json::from_reader(reader)
            .expect("Unable to serialize tasks from Json into struct Tasks")
    }

    pub fn add(&mut self, task: &Task,
    ) {
        self.tasks.push(task.clone());
    }

    pub fn save(&mut self, mut writer: impl std::io::Write) {
        serde_json::to_writer_pretty(writer, &self.tasks).expect("Unable to write data to writer")
    }
}

//#[cfg(test)]
//
// #[test]
// fn should_serialize_from_source() {
//     let data = r#"
//     {
//         "tasks": [
//             {
//                 "number": 1,
//                 "tags": "Home",
//                 "description": "Clean the kids room",
//                 "creation_date": "",
//                 "status": "ToDo"
//             }
//         ]
//     }"#;
//     struct TestData {
//         data: String,
//     };
//     impl Read for TestData {
//         fn get_data(&self) -> String {
//             (&self.data).to_string()
//         }
//     }
//     let source = TestData {
//         data: data.to_string(),
//     };
//     let wanted = "Clean the kids room".to_string();
//     let got = Tasks::from(source);
//     assert_eq!(got.tasks[0].description, wanted);
// }
