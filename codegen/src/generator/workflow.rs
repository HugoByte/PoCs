use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workflow {
    name: String,
    version: String,
}
