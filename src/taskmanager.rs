use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::io::{Read};
use chrono::Utc;
use chrono::FixedOffset;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tasks {
    pub tasks: Vec<Task>,
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

    pub fn add(&mut self, task: &Task) {
        let mut task_index = 0;
        let max_number = self.tasks.iter().map(|t| t.number).max();
        match max_number {
            Some(max) => task_index = max + 1,
            None => task_index = 1
        };
        println!("task number to use : {}", task_index);
        let mut task_to_add = task.clone();
        task_to_add.number = task_index;
        self.tasks.push(task_to_add);
    }

    pub fn save(&mut self, writer: impl std::io::Write) {
        serde_json::to_writer_pretty(writer, &self).expect("Unable to write data to writer")
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub number: u32,
    pub tags: Option<String>,
    pub description: String,
    pub creation_date: String,
    pub status: Status,
}

impl Task {
    pub fn create(description: String, tags: Option<String>, date: Option<DateTime<FixedOffset>>) -> Task {
        let current_date: String;
        match date {
            Some(d) => current_date = d.to_rfc3339(),
            None => current_date = Utc::now().to_rfc3339()
        };
            
        return Task {
            number: 0,
            tags: tags,
            description: description,
            creation_date: current_date,
            status: Status::ToDo
        };
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    ToDo,
    WIP,
    Done,
}

#[test]
fn should_add_a_task() {
    let mut tasks = Tasks { tasks: vec![] };
    let new_task = Task {
        number: 0,
        tags: None,
        description: "".to_string(),
        creation_date: "".to_string(),
        status: Status::ToDo,
    };
    tasks.add(&new_task);
    assert_eq!(tasks.tasks.len(), 1);
}


#[test]
fn should_create_a_task_with_current_date() {
    let fake_today = DateTime::parse_from_rfc3339("2020-07-25T16:39:57-08:00").unwrap();
    let task = Task::create("Test date".to_string(), Some("a, b, c".to_string()), Some(fake_today));
    assert_eq!(task.creation_date, fake_today.to_rfc3339());
}


#[test]
fn should_increment_id_when_adding_tasks() {
    let task1 = Task::create("Test 1".to_string(), Some("a, b, c".to_string()), None);
    let task2 = Task::create("Test 2".to_string(), Some("a, b, c".to_string()), None);

    let mut tasks = Tasks { tasks: vec![] };
    tasks.add(&task1);
    tasks.add(&task2);
    
    assert_eq!(tasks.tasks[0].number, 1);
    assert_eq!(tasks.tasks[1].number, 2);
     
}