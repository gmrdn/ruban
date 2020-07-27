use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::io::Read;

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
        let task_index;
        let max_number = self.tasks.iter().map(|t| t.number).max();
        match max_number {
            Some(max) => task_index = max + 1,
            None => task_index = 1,
        };
        println!("task number to use : {}", task_index);
        let mut task_to_add = task.clone();
        task_to_add.number = task_index;
        self.tasks.push(task_to_add);
    }

    pub fn remove(&mut self, number: u32) {
        let mut i = 0;
        while i != self.tasks.len() {
            if self.tasks[i].number == number {
                self.tasks.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn change_status_to(&mut self, number: u32, new_status: Status) {
        for i in 0..self.tasks.len() {
            if self.tasks[i].number == number {
                self.tasks[i].status = new_status
            }
        }
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
    pub fn create(
        description: String,
        tags: Option<String>,
        date: Option<DateTime<FixedOffset>>,
    ) -> Task {
        let current_date: String;
        match date {
            Some(d) => current_date = d.to_rfc3339(),
            None => current_date = Utc::now().to_rfc3339(),
        };

        Task {
            number: 0,
            tags,
            description,
            creation_date: current_date,
            status: Status::ToDo,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
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
    let task = Task::create(
        "Test date".to_string(),
        Some("a, b, c".to_string()),
        Some(fake_today),
    );
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

#[test]
fn should_remove_a_task_by_number() {
    let task1 = Task::create("Test 1".to_string(), Some("a, b, c".to_string()), None);
    let task2 = Task::create("Test 2".to_string(), Some("a, b, c".to_string()), None);

    let mut tasks = Tasks { tasks: vec![] };
    tasks.add(&task1);
    tasks.add(&task2);

    assert_eq!(tasks.tasks.len(), 2);
    assert_eq!(tasks.tasks[0].description, "Test 1".to_string());

    tasks.remove(1);
    assert_eq!(tasks.tasks.len(), 1);
    assert_eq!(tasks.tasks[0].description, "Test 2".to_string());
}

#[test]
fn should_move_a_task_to_wip() {
    let task1 = Task::create("Task to move".to_string(), None, None);
    let mut tasks = Tasks { tasks: vec![] };
    tasks.add(&task1);
    tasks.change_status_to(1, Status::WIP);
    assert_eq!(tasks.tasks[0].status, Status::WIP);
}
