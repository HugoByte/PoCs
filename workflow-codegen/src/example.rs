pub struct CartypeInput {
    car_type: String,
}

pub struct Cartype {
    action_name: String,
    pub input_args: CartypeInput,
    pub output: Value,
}

pub struct ModelavailInput {
    car_company_list: HashMap<String,Vec<String>>,
    company_name: String,
}

pub struct Modelavail {
    action_name: String,
    pub input_args: ModelavailInput,
    pub output: Value,
}

pub struct ModelspriceInput {
    models: Vec<String>,
}

pub struct Modelsprice {
    action_name: String,
    pub input_args: ModelspriceInput,
    pub output: Value,
}

pub struct PurchaseInput {
    model_price_list: HashMap<String,i32>,
    model_name: String,
    price: i32,
}

pub struct Purchase {
    action_name: String,
    pub input_args: PurchaseInput,
    pub output: Value,
}