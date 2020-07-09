pub mod rendering {
    use crate::Tasks;


    pub fn greet_the_user(
        mut writer: impl std::io::Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(writer, "Hello, Ruban User.")?;
        Ok(())
    }

    pub fn confirm_the_task(
        task: String,
        mut writer: impl std::io::Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(writer, "New task: {}", task)?;
        Ok(())
    }

    pub fn confirm_the_tags(
        tags: String,
        mut writer: impl std::io::Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(writer, "Tags: {}", tags)?;
        Ok(())
    }

    pub(crate) fn render_all_tasks(
        tasks: &Tasks,
        mut writer: impl std::io::Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(writer, "All tasks:")?;
        for task in tasks {
            writeln!(writer, "{} - {}", task.number, task.task)?;
        }
        Ok(())
    }

    #[cfg(test)]
    use crate::Task;
    #[cfg(test)]
    use std::str::from_utf8;

    #[test]
    fn should_greet_the_user() {
        let mut result = Vec::new();
        greet_the_user(&mut result);
        assert_eq!(from_utf8(&result).unwrap(), "Hello, Ruban User.\n");
    }

    #[test]
    fn should_confirm_the_task() {
        let task = String::from("Do the laundry");
        let mut result = Vec::new();
        confirm_the_task(task, &mut result);
        assert_eq!(from_utf8(&result).unwrap(), "New task: Do the laundry\n");
    }

    #[test]
    fn should_confirm_the_tags() {
        let tags = String::from("House");
        let mut result = Vec::new();
        confirm_the_tags(tags, &mut result);
        assert_eq!(from_utf8(&result).unwrap(), "Tags: House\n");
    }

    #[test]
    fn should_display_all_tasks() {
        let tasks = Tasks {
            tasks: vec![
                Task {
                    number: 1,
                    tags: Some("House".to_string()),
                    task: "Repair the garage door.".to_string(),
                    creation_date: None,
                    status: None,
                },
                Task {
                    number: 2,
                    tags: Some("Dev".to_string()),
                    task: "Finish the Rust Book.".to_string(),
                    creation_date: None,
                    status: None,
                },
            ],
        };
        let mut result = Vec::new();

        render_all_tasks(&tasks, &mut result);
        assert_eq!(
            from_utf8(&result).unwrap(),
            "All tasks:\n1 - Repair the garage door.\n2 - Finish the Rust Book.\n"
        );
    }
}
