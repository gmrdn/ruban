use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Cli {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub number: u32,
    pub tags: Option<String>,
    pub task: String,
    pub creation_date: Option<String>,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    WIP,
    Done,
}
