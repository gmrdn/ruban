mod adapter_dataprovider;
mod adapter_user_interface;
mod port_dataprovider;
mod rendering;
mod taskmanager;

//use crate::adapter_dataprovider::DataFile;
use crate::adapter_user_interface::Cli;
use crate::rendering::{confirm_the_task, greet_the_user, render_all_tasks};
use crate::taskmanager::{Status, Task, Tasks};
use chrono::Utc;
use structopt::StructOpt;
use std::fs::{File, OpenOptions};

const STD_OUT_ERR_MSG: &str = "Unable to write message in standard output";

fn main() {
    let source = OpenOptions::new()
        .read(true)
        .write(true)
        .open("test_tasks.json")
        .expect("Unable to open file");


    let destination = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("output.json")
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
            //let file = File::create("foo.txt").expect("Unable to create file");

            tasks.add(&task);
            tasks.save(&destination);
            confirm_the_task(&task, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
            render_all_tasks(&tasks, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
        }
        Cli::Ls {} => {
            render_all_tasks(&tasks, &mut std::io::stdout()).expect(STD_OUT_ERR_MSG);
        }
        _ => (),
    }
}
