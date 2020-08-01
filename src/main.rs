mod cli;
mod rendering;
mod taskmanager;

extern crate prettytable;
extern crate dirs;


use crate::cli::Cli;
use crate::rendering::{
    confirm_task_moved, confirm_task_removed, confirm_the_task, greet_the_user, render_all_tasks,
};
use crate::taskmanager::{Status, Task, Tasks};
use chrono::Utc;
use std::fs::OpenOptions;
use structopt::StructOpt;
use std::path::PathBuf;
use std::str::FromStr;

const STD_OUT_ERR_MSG: &str = "Unable to write message in standard output";

fn main() {

    let filepath: PathBuf = [dirs::data_local_dir().unwrap(), PathBuf::from_str("ruban.json").unwrap()].iter().collect::<PathBuf>();


    let source = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filepath.as_os_str())
        .expect("Unable to open file");

    let destination = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filepath.as_os_str())
        .expect("Unable to open file");

    let mut tasks = Tasks::from(&source);

    greet_the_user(&mut std::io::stdout()).expect(STD_OUT_ERR_MSG);

    match Cli::from_args() {
        Cli::Add { description, tags } => {
            let task = Task {
                number: 0,
                tags: Some(tags),
                description,
                creation_date: Utc::now().to_rfc3339(),
                status: Status::ToDo,
            };
            tasks.add(&task);

            destination
                .set_len(0)
                .expect("Unable to clear content from file");
            tasks.save(&destination);
            confirm_the_task(&task, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
            render_all_tasks(&tasks, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
        }
        Cli::Ls {} => {
            render_all_tasks(&tasks, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
        }
        Cli::Rm { number } => {
            tasks.remove(number);
            destination
                .set_len(0)
                .expect("Unable to clear content from file");
            tasks.save(&destination);
            confirm_task_removed(number, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
            render_all_tasks(&tasks, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
        }
        Cli::Mv { number, status } => {
            let new_status: Status;
            match status.to_lowercase().as_str() {
                "wip" => new_status = Status::WIP,
                "todo" => new_status = Status::ToDo,
                "done" => new_status = Status::Done,
                _ => new_status = Status::WIP,
            };
            tasks.change_status_to(number, new_status);
            destination
                .set_len(0)
                .expect("Unable to clear content from file");
            tasks.save(&destination);
            confirm_task_moved(number, &status, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
            render_all_tasks(&tasks, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
        }
    }
}
