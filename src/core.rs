use crate::port_dataprovider::DataProvider;
use crate::types::Tasks;

pub fn retrieve_tasks(source: impl DataProvider) -> Tasks {
    let data_from_file = source.get_data();

    let tasks_from_json: Tasks = serde_json::from_str(data_from_file.as_str())
        .expect("Unable to serialize tasks from Json into struct Tasks");
    tasks_from_json
}

#[test]
fn should_retrieve_tasks() {
    let data = r#"
    {
        "tasks": [
            {
                "number": 1,
                "tags": "Home",
                "task": "Clean the kids room",
                "creation_date": "",
                "status": "ToDo"
            }
        ]
    }"#;
    struct TestData {
        data: String,
    };
    impl DataProvider for TestData {
        fn get_data(&self) -> String {
            (&self.data).to_string()
        }
    }
    let source = TestData {
        data: data.to_string(),
    };
    let wanted = "Clean the kids room".to_string();
    let got = retrieve_tasks(source);
    assert_eq!(got.tasks[0].task, wanted);
}
