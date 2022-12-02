use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub tasks: Vec<Task>,
    pub flows: Vec<Flow>,
    pub workflow: Workflow,
}

impl Config {
    pub fn generate_code(&self) {
        let mut scope = Scope::new();
        let mut variants: Vec<Variant> = vec![];
        self.tasks.iter().for_each(|task| {
            let task_code = task.generate_task();

            scope.raw(task_code);

            variants.push(task.generate_variant());
        });
        let en = self.generate_enum(variants);

        scope.raw(en);
        let file = fs::File::create("./src/example.rs");
        file.unwrap()
            .write_all(scope.to_string().as_bytes())
            .unwrap();
    }
    fn generate_enum(&self, variants: Vec<Variant>) -> String {
        let mut enum_type = Enum::new("Types");
        enum_type.push_variant(Variant::new("Empty(String)"));
        enum_type.push_variant(Variant::new("Concat(Vec<Types>)"));

        variants.into_iter().for_each(|v| {
            enum_type.push_variant(v);
        });

        let mut scope = Scope::new();
        scope.push_enum(enum_type.vis("pub").to_owned());

        scope.to_string()
    }

    pub fn generate_dependecy(&self) {
        for (index, flow) in self.flows.iter().enumerate() {
            if flow.flow_type == "Init"
                && flow.task_name == self.tasks[index].name
                && flow.depends_on == None
            {
            //    self.tasks[index].
            }
        }
    }
}
