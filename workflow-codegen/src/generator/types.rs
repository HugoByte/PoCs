use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub tasks: Vec<Task>,
    pub flows: Vec<Flow>,
    pub workflow: Workflow,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configs {
    pub workflow: Workflows,
}

impl Configs {
    pub fn generate_code(&self) {
        let mut scope = Scope::new();

        self.workflow.tasks.iter().for_each(|task| {
            let task_code = task.generate_task();

            scope.raw(task_code);
        });

        let file = fs::File::create("./src/example.rs");
        file.unwrap()
            .write_all(scope.to_string().as_bytes())
            .unwrap();
    }

    pub fn generate_dependecy(&self) {}
}
