use hex;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Counter {
    pub count: i32,
}

pub const STATE: Item<Counter> = Item::new("state");
