pub mod generator;

pub use convert_case::*;
#[cfg(test)]
mod tests {

    use super::*;
    use generator::*;

    #[test]
    fn read_file() {
        let path = "config2.yaml".to_string();

        let file_contents = generator::read_yaml_file(path);

        let config: Configs = serde_yaml::from_str(&file_contents).unwrap();

        let r = config.workflow.generate_dependecy();
        println!("{:#?}", r);
    }
}
