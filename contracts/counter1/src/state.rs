use hex;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct College {
    pub students: Students,
}

pub const STATE: Item<College> = Item::new("state");

type Students = Vec<Student>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Student {
    id: String,
    name: String,
    age: u8,
    gender: String,
    semister: String,
    blood_group: String,
}

impl Student {
    pub fn new(
        name: String,
        age: u8,
        semister: String,
        blood_group: String,
        gender: String,
    ) -> Self {
        let id = hex::encode(name.clone());
        Self {
            id,
            name,
            age,
            gender,
            semister,
            blood_group,
        }
    }

    pub fn age(&self) -> u8 {
        self.age
    }
    pub fn gender(&self) -> &String {
        &self.gender
    }

    pub fn semister(&self) -> &String {
        &self.semister
    }

    pub fn blood_group(&self) -> &String {
        &self.blood_group
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
