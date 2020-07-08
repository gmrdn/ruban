use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "", short = "t", long = "tags")]
    tags: String,
    task: String,
}


fn main() {
    let args = Cli::from_args();
    greet_the_user(&mut std::io::stdout());
    confirm_the_task(args.task, &mut std::io::stdout());
    confirm_the_tags(args.tags, &mut std::io::stdout());
}

fn greet_the_user(mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    match writeln!(writer, "Hello, Ruban User.") {
        Ok(content) => Ok(()),
        Err(error) => { return Err(error.into()); }
    }
}

fn confirm_the_task(task: String, mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    match writeln!(writer, "New task: {}", task) {
        Ok(content) => Ok(()),
        Err(error) => { return Err(error.into()); }
    }
}


fn confirm_the_tags(tags: String, mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    match writeln!(writer, "Tags: {}", tags) {
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
    assert_eq!(result, b"New task: etendre le linge\n");
}


#[test]
fn should_confirm_the_tags() {
    let tags = String::from("Maison");
    let mut result = Vec::new();
    confirm_the_tags(tags, &mut result);
    assert_eq!(result, b"Tags: Maison\n");
}
