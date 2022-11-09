use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use wasi_common::WasiCtx;
use std::{collections::HashMap, sync::{Mutex, Arc}};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
mod wasi_http;
use std::convert::TryInto;
use wasi_http::HttpCtx;

pub static WASM_FILE: &'static [u8] = include_bytes!("../action_purchase_workflow.wasm");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    allowed_hosts: Option<Vec<String>>,
    data: Value,
}

// #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
// struct Input {
//     value: Value,
//     #[serde(flatten)]
//     environment: HashMap<String, Value>,
// }

pub fn main(args: Value) -> Result<Value, Error> {
    // let input: MainInput = serde_json::from_value(serde_json::json!({
    //     "allowed_hosts": [
    //       "https://westend-rpc.polkadot.io", "https://65.20.70.146:31001"
    //     ],
    //     "data": {  
    //         "validator_address"  : "5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f",
    //         "owner_key" : "sting piano series slot blast retire victory pistol eye magnet exotic found",
    //         "end_point": "https://westend-rpc.polkadot.io",
    //     }
    // }))
    // .unwrap();
    let input: Input = serde_json::from_value(args)?;

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);

    let output: Arc<Mutex<Output>> = Arc::new(Mutex::new(Output {
        result: serde_json::json!({}),
    }));
    let output_ = output.clone();
    let buf = serde_json::to_vec(&input).expect("should serialize");
    let mem_size: i32 = buf.len() as i32;

    linker
        .func_wrap("host", "get_input_size", move || -> i32 { mem_size })
        .expect("should define the function");

    linker
        .func_wrap(
            "host",
            "set_output",
            move |mut caller: Caller<'_, WasiCtx>, ptr: i32, capacity: i32| {
                let output = output_.clone();
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => return Err(Trap::new("failed to find host memory")),
                };
                let offset = ptr as u32 as usize;
                let mut buffer: Vec<u8> = vec![0; capacity as usize];
                match mem.read(&caller, offset, &mut buffer) {
                    Ok(()) => {
                        println!(
                            "Buffer = {:?}, ptr = {}, capacity = {}",
                            buffer, ptr, capacity
                        );
                        match serde_json::from_slice::<Output>(&buffer) {
                            Ok(serialized_output) => {
                                let mut output = output.lock().unwrap();
                                *output = serialized_output;
                                Ok(())
                            }
                            Err(err) => {
                                let msg = format!("failed to serialize host memory: {}", err);
                                Err(Trap::new(msg))
                            }
                        }
                    }
                    _ => Err(Trap::new("failed to read host memory")),
                }
            },
        )
        .expect("should define the function");

    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);
    let module = Module::from_binary(&engine, WASM_FILE).unwrap();
    let max_concurrent_requests = Some(42);

    let http = HttpCtx::new(input.allowed_hosts, max_concurrent_requests).unwrap();
    http.add_to_linker(&mut linker).unwrap();

    let linking = linker.instantiate(&mut store, &module).unwrap();

    let malloc = linking
        .get_typed_func::<(i32, i32), i32, _>(&mut store, "memory_alloc")
        .unwrap();
    let data = serde_json::to_vec(&input.data).unwrap();
    let data_ptr = malloc.call(&mut store, (data.len() as i32, 2)).unwrap();

    let memory = linking.get_memory(&mut store, "memory").unwrap();
    memory.write(&mut store, data_ptr as usize, &data).unwrap();
    let len: i32 = data.len().try_into().unwrap();
    let run = linking
        .get_typed_func::<(i32, i32), (), _>(&mut store, "_start")
        .unwrap();
    let _result_from_wasm = run.call(&mut store, (data_ptr, len));

    

    let malloc = linking
        .get_typed_func::<(i32, i32, i32), (), _>(&mut store, "free_memory")
        .unwrap();
    malloc
        .call(&mut store, (data_ptr, data.len() as i32, 2))
        .unwrap();
    let output = output.lock();
    // println!("output: {:?}", output.unwrap().result);
    Ok(serde_json::json!({"result": output.unwrap().result}))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub result: Value,
}


// https://65.20.70.146:31001/api/v1/namespaces/guest/actions/payout_conatiner_test_with_ex?blocking=true&result=true

#[test]
fn testing_main(){
    let serde_json = serde_json::json!({
        "allowed_hosts": [
             "https://65.20.70.146:31001", "https://65.20.70.146:31001/api/v1/namespaces/guest/actions/cartype?blocking=true&result=true", "https://65.20.70.146:31001/api/v1/namespaces/guest/actions/modelavail?blocking=true&result=true"
        ],
        "data": {  
            "car_type": "hatchback",
            "company_name": "maruthi",
        }
    
    });
    let res =main(serde_json);
    println!("{:?}", res);
}