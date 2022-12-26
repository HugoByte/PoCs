use std::str::FromStr;

use fvm_shared::{address::Address, econ::TokenAmount};

use super::*;

/// Method num 2.
pub fn say_hello() -> Option<RawBytes> {
    let mut state = State::load();
    state.count += 1;
    state.save();

    let caller = sdk::message::caller();
    let origin = sdk::message::origin();
    let receiver = sdk::message::receiver();

    let ret = to_vec(
        format!(
            "Hello world {caller}/{origin}/{receiver} #{}!",
            &state.count
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
