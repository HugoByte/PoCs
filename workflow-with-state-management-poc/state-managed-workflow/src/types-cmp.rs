use super::*;
use openwhisk_macro::*;
use openwhisk_rust::*;

make_input_struct!(
ModelavailInput,
[car_company_list:HashMap<String,Vec<String>>,company_name:String],
[Debug, Clone, Default, Serialize, Deserialize]
);
make_input_struct!(
PurchaseInput,
[model_price_list:HashMap<String,i32>,model_name:String,price:i32],
[Debug, Clone, Default, Serialize, Deserialize]
);
make_input_struct!(
CartypeInput,
[car_type:String],
[Debug, Clone, Default, Serialize, Deserialize]
);
make_input_struct!(
ModelspriceInput,
[models:Vec<String>],
[Debug, Clone, Default, Serialize, Deserialize]
);

make_main_struct!(
    Modelavail,
    ModelavailInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",ApiHost:"http://127.0.0.1:8080",Insecure:"true",Namespace:"guest"],
    output
);
impl_new!(
    Modelavail,
    ModelavailInput,
    [company_name:String]
);

make_main_struct!(
    Purchase,
    PurchaseInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [Namespace:"guest",AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Insecure:"true",ApiHost:"http://127.0.0.1:8080"],
    output
);
impl_new!(
    Purchase,
    PurchaseInput,
    [model_name:String,price:i32]
);

make_main_struct!(
    Cartype,
    CartypeInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Namespace:"guest",Insecure:"true",ApiHost:"http://127.0.0.1:8080"],
    output
);
impl_new!(
    Cartype,
    CartypeInput,
    [car_type:String]
);

make_main_struct!(
    Modelsprice,
    ModelspriceInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",ApiHost:"http://127.0.0.1:8080",Insecure:"true",Namespace:"guest"],
    output
);
impl_new!(Modelsprice, ModelspriceInput, []);

impl_setter!(Modelavail, [car_company_list:"car_company_list"]);
impl_setter!(Purchase, [model_price_list:"model_price_list"]);
impl_setter!(Cartype, []);
impl_setter!(Modelsprice, [models:"models"]);

pub fn car_type_fn() -> String {
    "tesla".to_string()
}

make_input_struct!(
Input,
[company_name:String,model_name:String,price:i32,#["car_type_fn"] car_type:String],
[Debug, Clone, Default, Serialize, Deserialize]
);
impl_execute_trait!(Modelavail, Purchase, Cartype, Modelsprice);
#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {
    const LIMIT: usize = 4;
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;

    let modelavail = Modelavail::new(input.company_name, "modelavail".to_string());
    let purchase = Purchase::new(input.model_name, input.price, "purchase".to_string());
    let cartype = Cartype::new(input.car_type, "cartype".to_string());
    let modelsprice = Modelsprice::new("modelsprice".to_string());

    let cartype_index = workflow.add_node(Box::new(cartype));
    let modelavail_index = workflow.add_node(Box::new(modelavail));
    let modelsprice_index = workflow.add_node(Box::new(modelsprice));
    let purchase_index = workflow.add_node(Box::new(purchase));

    workflow.add_edges(&[
        (cartype_index, modelavail_index),
        (modelavail_index, modelsprice_index),
        (modelsprice_index, purchase_index),
    ]);
    let result = workflow
        .pipe(cartype_index)?
        .pipe(modelavail_index)?
        .pipe(modelsprice_index)?
        .pipe(purchase_index)?;


    let len = workflow.node_count();
    let output = workflow.get_task(len - 1).get_task_output();

    let result = serde_json::to_value(output).unwrap();
    Ok(result)
}
