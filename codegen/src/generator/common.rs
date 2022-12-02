use super::*;
pub fn read_yaml_file(path: String) -> String {
    fs::read_to_string(path).expect("Unable to read the file")
}

pub fn convert_to_pascal(s: &str) -> String {
    s.to_case(Case::Pascal).to_string()
}
