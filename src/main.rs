use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    task: String,
}


fn main() {
    let args = Cli::from_args();
    greet_the_user(&mut std::io::stdout());
    confirm_the_task(args.task, &mut std::io::stdout());
}

fn greet_the_user(mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    match writeln!(writer, "Hello, Ruban User.") {
        Ok(content) => Ok(()),
        Err(error) => { return Err(error.into()); }
    }
}

fn confirm_the_task(task: String, mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    match writeln!(writer, "{}", task) {
        Ok(content) => Ok(()),
        Err(error) => { return Err(error.into()); }
    }
}

#[test]
fn should_greet_the_user() {
    let mut result = Vec::new();
    greet_the_user(&mut result);
    assert_eq!(result, b"Hello, Ruban User.\n");
}

#[test]
fn should_confirm_the_task() {
    let task = String::from("etendre le linge");
    let mut result = Vec::new();
    confirm_the_task(task, &mut result);
    assert_eq!(result, b"etendre le linge\n");
}
