use std::borrow::Borrow;

use super::*;
use openwhisk_macro::*;
use openwhisk_rust::*;

make_input_struct!(
EmployeeIdsInput,
[role:String],
[Debug, Clone, Default, Serialize, Deserialize]
);
make_input_struct!(
GetsalariesInput,
[id:i32],
[Debug, Clone, Default, Serialize, Deserialize]
);
make_input_struct!(
SalaryInput,
[details:HashMap<i32,(i32,String)>],
[Debug, Clone, Default, Serialize, Deserialize]
);
make_input_struct!(
GetaddressInput,
[id:i32],
[Debug, Clone, Default, Serialize, Deserialize]
);

make_main_struct!(
    EmployeeIds,
    EmployeeIdsInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Insecure:"true",Namespace:"guest",ApiHost:"http://127.0.0.1:1234"],
    output
);
impl_new!(
    EmployeeIds,
    EmployeeIdsInput,
    [role:String]
);

make_main_struct!(
    Getsalaries,
    GetsalariesInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [Insecure:"true",AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Namespace:"guest",ApiHost:"http://127.0.0.1:1234"],
    mapout
);
impl_new!(Getsalaries, GetsalariesInput, []);

make_main_struct!(
    Salary,
    SalaryInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [ApiHost:"http://127.0.0.1:1234",Namespace:"guest",Insecure:"true",AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"],
    output
);
impl_new!(Salary, SalaryInput, []);

make_main_struct!(
    Getaddress,
    GetaddressInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [Namespace:"guest",ApiHost:"http://127.0.0.1:1234",Insecure:"true",AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"],
    mapout
);
impl_new!(Getaddress, GetaddressInput, []);

impl_setter!(EmployeeIds, []);
impl_map_setter!(Getsalaries, id:"ids", i32, "salary");
impl_concat_setter!(Salary, details);
impl_map_setter!(Getaddress, id:"ids", i32, "address");

make_input_struct!(
Input,
[role:String],
[Debug, Clone, Default, Serialize, Deserialize]
);
impl_execute_trait!(EmployeeIds, Getsalaries, Salary, Getaddress);

#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {
    const LIMIT: usize = 4;
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;

    let employee_ids = EmployeeIds::new(input.role, "employee_ids".to_string());
    let getsalaries = Getsalaries::new("getsalaries".to_string());
    let salary = Salary::new("salary".to_string());
    let getaddress = Getaddress::new("getaddress".to_string());

    let employee_ids_index = workflow.add_node(Box::new(employee_ids));
    let getsalaries_index = workflow.add_node(Box::new(getsalaries));
    let getaddress_index = workflow.add_node(Box::new(getaddress));
    let salary_index = workflow.add_node(Box::new(salary));

    workflow.add_edges(&[
        (employee_ids_index, getsalaries_index),
        (employee_ids_index, getaddress_index),
        (getsalaries_index, salary_index),
        (getaddress_index, salary_index),
    ]);

    // let result = workflow
    //     .init()?
    //     .pipe(getsalaries_index)?
    //     .pipe(getaddress_index)?
    //     .pipe(salary_index)?
    //     .term(None)?;    // salary is depending and term does not handle multiple deps

    let mut internal_state_data = StateData::init("employee_ids");

    let mut result = match workflow.init() {
        Ok(res) => {
            internal_state_data.update("getsalaries", getsalaries_index);
            res
        }
        Err(err) => {
            internal_state_data.update_err(&err);
            return Err(err);
        }
    };

    let result = match result.pipe(getsalaries_index) {
        Ok(res) => {
            internal_state_data.update("getaddress", getaddress_index);
            res
        }
        Err(err) => {
            internal_state_data.update_err(&err);
            return Err(err);
        }
    };

    let result = match result.pipe(getaddress_index) {
        Ok(res) => {
            internal_state_data.update("salary", salary_index);
            res
        }
        Err(err) => {
            internal_state_data.update_err(&err);
            return Err(err);
        }
    };

    let result = match result.pipe(salary_index) {
        Ok(res) => {
            // internal_state_data.update("salary", getsalaries_index);     // there is no task left to execute
            res
        }
        Err(err) => {
            internal_state_data.update_err(&err);
            return Err(err);
        }
    };

    // simply returns the output
    let result = result.term(None)?;
    let result = serde_json::to_value(result).unwrap();
    Ok(result)
}
