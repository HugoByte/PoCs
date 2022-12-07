mod common;
mod flow;
mod task;
mod types;
mod workflow;

pub use common::*;
pub use flow::*;
pub use task::*;
pub use types::*;
pub use workflow::*;

use serde::{Deserialize, Serialize};

use codegen::Scope;
use std::fs;

use codegen::{Enum, Field, Struct, Variant};
use convert_case::{Case, Casing};

use std::collections::HashMap;
use std::io::Write;
