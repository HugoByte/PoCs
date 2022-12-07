use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub kind: String,
    pub name: String,
    pub input_args: Vec<Args>,
    pub properties: HashMap<String, String>,
    pub flow: Flow,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Args {
    pub name: String,
    pub arg_type: String,
}

impl Task {
    pub fn generate_task(&self) -> String {
        let mut scope = Scope::new();
        let task_name = convert_to_pascal(&self.name);
        let mut input_struct = self.generate_struct(&task_name);

        scope.push_struct(input_struct.vis("pub").to_owned());

        let mut main_struct = Struct::new(&task_name);
        main_struct.field("action_name", "String");

        let input_field = Field::new("input_args", format!("{}Input", task_name).as_str())
            .vis("pub")
            .to_owned();
        let output_filed = Field::new("output", "Value").vis("pub").to_owned();

        main_struct.push_field(input_field);
        main_struct.push_field(output_filed);
        main_struct.vis("pub");

        scope.push_struct(main_struct);

        scope.to_string()
    }

    fn generate_struct(&self, name: &str) -> Struct {
        let mut input_struct = Struct::new(format!("{}Input", name).as_str());

        self.input_args.iter().for_each(|args| {
            let field = Field::new(&args.name, &args.arg_type);

            input_struct.push_field(field);
        });

        input_struct
    }

    pub fn generate_impl_for_task(&self) -> String {
        let task_name = convert_to_pascal(&self.name);
        let mut scope = Scope::new();

        let method_impl = scope.new_impl(&task_name);

        "".to_string()
    }
}
