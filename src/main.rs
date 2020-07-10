mod adapter_dataprovider;
mod core;
mod port_dataprovider;
mod rendering;
mod types;

use crate::adapter_dataprovider::DataFile;
use crate::core::retrieve_tasks;
use crate::rendering::{confirm_the_tags, confirm_the_task, greet_the_user, render_all_tasks};
use crate::types::Cli;
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
