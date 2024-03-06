use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(test)]
mod wasi_http;
mod types;

pub use types::*;
pub mod helper;
pub use helper::*;


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use std::{
        fs,
        sync::{Arc, Mutex},
    };
    use wasi_common::WasiCtx;
    use wasi_http::HttpCtx;
    use wasmtime::Linker;
    use wasmtime::*;
    use wasmtime_wasi::sync::WasiCtxBuilder;
    use std::collections::HashMap;

    #[allow(dead_code)]
    fn run_workflow(data: Value, path: String) -> (Output, Vec<StateData>) {
        let wasm_file = fs::read(path).unwrap();
        let input: MainInput = serde_json::from_value(data).unwrap();
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
                        Ok(()) => match serde_json::from_slice::<Output>(&buffer) {
                            Ok(serialized_output) => {
                                let mut output = output.lock().unwrap();
                                *output = serialized_output;
                                Ok(())
                            }
                            Err(err) => {
                                let msg = format!("failed to serialize host memory: {}", err);
                                Err(Trap::new(msg))
                            }
                        },
                        _ => Err(Trap::new("failed to read host memory")),
                    }
                },
            )
            .expect("should define the function");

        let output_2: Arc<Mutex<Vec<StateData>>> = Arc::new(Mutex::new(Vec::new()));

        let output_ = output_2.clone();

        linker
            .func_wrap(
                "host",
                "set_state",
                move |mut caller: Caller<'_, WasiCtx>, ptr: i32, capacity: i32| {
                    let output_2 = output_.clone();
                    let mem = match caller.get_export("memory") {
                        Some(Extern::Memory(mem)) => mem,
                        _ => return Err(Trap::new("failed to find host memory")),
                    };
                    let offset = ptr as u32 as usize;
                    let mut buffer: Vec<u8> = vec![0; capacity as usize];
                    match mem.read(&caller, offset, &mut buffer) {
                        Ok(()) => match serde_json::from_slice::<StateData>(&buffer) {
                            Ok(serialized_output) => {
                                let mut output_2 = output_2.lock().unwrap();
                                output_2.push(serialized_output);
                                Ok(())
                            }
                            Err(err) => {
                                let msg = format!("failed to serialize host memory: {}", err);
                                Err(Trap::new(msg))
                            }
                        },
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
        let module = Module::from_binary(&engine, &wasm_file).unwrap();
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


        let state_output = output_2.lock().unwrap().clone();
        let res = output.lock().unwrap().clone();
        (res, state_output)
    }


    #[async_std::test]
    async fn test_employee_salary_with_concat_operator() {
        let path = std::env::var("WORKFLOW_WASM").unwrap_or(format!(
            "../state-managed-workflow/target/wasm32-wasi/release/boilerplate.wasm"
        ));
        let server = post("127.0.0.1:1234").await;
        let input = serde_json::json!({
            "allowed_hosts": [
                server.uri()
            ],
            "data": {
                "role":"Software Developer",
                }
        });

        let (result, state_data) = run_workflow(input, path);

        // println!("State_data => {:#?}", state_data);

        let mut outputs: HashMap<String, Value> = HashMap::new();

        for sd in state_data {
            if sd.is_success(){
                outputs.insert(sd.get_action_name(), sd.get_output());
            }
        }

        println!("Outputs => {:#?}", outputs);

        // assert!(result
        //     .result
        //     .to_string()
        //     .contains("Salary creditted for emp id 1 from Hugobyte"))
    }

    #[async_std::test]
    async fn test_car_market_place() {
        let path = std::env::var("WORKFLOW_WASM").unwrap_or(format!(
            "../state-managed-workflow/target/wasm32-wasi/release/boilerplate.wasm"
        ));

        let server = post("127.0.0.1:8080").await;
        let input = serde_json::json!({
            "allowed_hosts": [
                server.uri()
            ],
            "data": {
                "car_type":"hatchback",
                "company_name":"maruthi",
                "model_name":"alto",
                "price":1200000
                }
        });
        let (result, _state_data) = run_workflow(input, path);

        println!("State_data => {:#?}", _state_data);

        assert!(result
            .result
            .to_string()
            .contains("Thank you for the purchase"))
    }

}
