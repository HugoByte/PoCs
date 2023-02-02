use std::str::FromStr;

use fvm_shared::{address::Address, econ::TokenAmount};
use serde::{Deserialize, Serialize};
use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Hello {
    pub name: String,
}

///Method 2
pub fn say_hello(params : u32) -> Option<RawBytes> {
    sdk::vm::set_panic_handler();

    let params = sdk::message::params_raw(params).unwrap().1;
    let params: Hello = serde_json::from_slice(&params).unwrap();

    let ret = to_vec(
        format!(
            "Hello world  {}", params.name
        )
        .as_str(),
    );
    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}
/// Method 3
pub fn call_hello() -> Option<RawBytes> {
    // let a = Address::from_str("f01002");  both are fine
    let a = Address::from_str("f2g25mp47ixng7frzchiiwgaushvmrpylsmy5ehba");
    let b = match a {
        Ok(s) => s,
        Err(_) => panic!("Erroroo rerro"),
    };

    let t = TokenAmount::default();
    let call = sdk::send::send(&b, 2, RawBytes::new(Vec::new()), t);
    println!("{:?}", call);
    


    let ret = to_vec(
        format!(
            "Hello world {:?}", call
        )
        .as_str(),
    );


    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}
