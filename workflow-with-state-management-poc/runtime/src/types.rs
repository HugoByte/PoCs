use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ExecutionState {
    // Init,
    Running,
    Aborted,
    Success,
}

impl Default for ExecutionState {
    fn default() -> Self {
        ExecutionState::Running
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename= "StateManager")]
pub struct StateData {
    // execution_state: ExecutionState, // to represent the task life cycle
    action_name: String,   // task name
    task_index: isize,     // n'th task out of m tasks
    execution_state: ExecutionState,
    output: Option<Value>,
    error: Option<String>, // to define the error kind
}

impl StateData{
    pub fn is_success(&self) -> bool {
        self.execution_state == ExecutionState::Success && self.task_index > -1
    }

    pub fn get_output(&self) -> Value {
        self.output.clone().into()
    }

    pub fn get_action_name(&self) -> String {
        self.action_name.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub result: Value,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Resultss {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainInput {
    pub allowed_hosts: Option<Vec<String>>,
    pub data: Value,
}