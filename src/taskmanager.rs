use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};
use chrono::Utc;
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::offset::TimeZone;
use std::str::FromStr;

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
        self.tasks.push(task.clone());
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
    pub fn create(date: Option<DateTime<FixedOffset>>) -> Task {
        let current_date: String;
        match date {
            Some(d) => current_date = d.to_rfc3339(),
            None => current_date = Utc::now().to_rfc3339()
        };
            
        return Task {
            number: 0,
            tags: None,
            description: "".to_string(),
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
    let task = Task::create(Some(fake_today));
    assert_eq!(task.creation_date, fake_today.to_rfc3339());
}