pub mod generator;

pub use convert_case::*;
#[cfg(test)]
mod tests {

    use super::*;
    use codegen::Scope;
    use generator::*;

    #[test]
    fn read_file() {
        let path = "example_config.yaml".to_string();

        let file_contents = generator::read_yaml_file(path);

        let config: Config = serde_yaml::from_str(&file_contents).unwrap();

        let mut scope = Scope::new();

        // TODO: WIP
        // let task = config.tasks[0].clone();

        // scope
        //     .new_struct(&task.name)
        //     .vis("pub")
        //     .new_field(&task.input_args[0].name, &task.input_args[0].arg_type)
        //     .vis("pub");

        // println!("{}", scope.to_string());

        config.generate_dependecy()
    }
}
