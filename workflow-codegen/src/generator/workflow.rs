use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workflow {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workflows {
    name: String,
    version: String,
    pub tasks: Vec<Task>,
}
impl Workflows {
    pub fn generate_dependecy(&self) -> HashMap<String, (String, Option<Vec<DependentTask>>)> {
        let mut dependency_matrix = HashMap::new();

        self.tasks
            .iter()
            .for_each(|task| match task.flow.depends_on.clone() {
                Some(dt) => {
                    dependency_matrix.insert(
                        task.name.clone(),
                        (task.flow.flow_type.clone(), Some(dt.task)),
                    );
                }
                None => {
                    dependency_matrix
                        .insert(task.name.clone(), (task.flow.flow_type.clone(), None));
                }
            });

        dependency_matrix
    }
}
