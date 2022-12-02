use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Flow {
    pub flow_type: String,
    pub task_name: String,
    pub depends_on: Option<DependsOn>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DependsOn {
    pub operation: Option<String>,
    pub task: Vec<DependentTask>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DependentTask {
    pub name: String,
    pub fields: Vec<String>,
}
