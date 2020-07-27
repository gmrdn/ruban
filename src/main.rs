mod adapter_dataprovider;
mod adapter_user_interface;
mod port_dataprovider;
mod rendering;
mod taskmanager;

#[macro_use] extern crate prettytable;


use crate::adapter_user_interface::Cli;
use crate::rendering::{confirm_task_removed, confirm_the_task, confirm_task_moved, greet_the_user, render_all_tasks};
use crate::taskmanager::{Status, Task, Tasks};
use chrono::Utc;
use std::fs::OpenOptions;
use structopt::StructOpt;

const STD_OUT_ERR_MSG: &str = "Unable to write message in standard output";

fn main() {
    let source = OpenOptions::new()
        .read(true)
        .write(true)
        .open("tasks.json")
        .expect("Unable to open file");

    let destination = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("tasks.json")
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
        }
        Cli::Mv { number, status} => {
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

        }
    }
}
