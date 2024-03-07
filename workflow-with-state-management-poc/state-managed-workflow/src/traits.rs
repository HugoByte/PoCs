use super::*;

pub trait Execute: Debug + DynClone {
    fn execute(&mut self) -> Result<(), String>;
    fn get_task_output(&self) -> Value;
    fn set_output_to_task(&mut self, inp: Value);
    fn get_action_name(&self) -> String;
}

clone_trait_object!(Execute);
