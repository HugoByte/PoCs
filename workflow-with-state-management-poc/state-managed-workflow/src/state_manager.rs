use super::*;
use crate::WorkflowGraph;

#[derive(Default, Debug)]
pub struct StateData{
    // execution_state: ExecutionState, // to represent the task life cycle 
    action_name: String, // task name
    task_index: usize, // n'th task out of m tasks
    error: Option<String> // to define the error kind 
}

impl StateData{

    fn update_state_data(&self){

       let state_data:serde_json::Value  =  serde_json::from_str(&format!("{{\"action_name\":\"{}\",\"task_index\":{}, \"error\":\"{:?}\"}}", self.action_name, self.task_index, self.error)).unwrap();

        let serialized = serde_json::to_vec(&state_data).unwrap();
        let size = serialized.len() as i32;
        let ptr = serialized.as_ptr();

        std::mem::forget(ptr);

        unsafe {
            super::set_state(ptr as i32, size);
        }
    }

    pub fn init(initial_task: &str) -> Self{
        let state_data = StateData{
            action_name: initial_task.to_string(),
            task_index: 0,
            error: None
        };

        state_data.update_state_data();

        state_data
    }

    pub fn update(&mut self, action_name: &str, task_index: usize){
        self.action_name = action_name.to_string();
        self.task_index = task_index;
        self.error = None;
        self.update_state_data();
    }

    pub fn update_err(&mut self, error: &str){
        self.error = Some(error.to_string());
        self.update_state_data();
    }

}