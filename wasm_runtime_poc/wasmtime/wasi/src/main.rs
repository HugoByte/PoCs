use anyhow::{Result, Ok};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasi_experimental_http_wasmtime::HttpCtx;



fn main() -> Result<()> {
    let engine = Engine::default();

    // First set up our linker which is going to be linking modules together. We
    // want our linker to have wasi available, so we set that up here as well.
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
    let linking = Module::from_file(&engine, "/Users/shreyas/Hugobyte/wasmtime/wasmtime.wasm")?;
   
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    let allowed_hosts = Some(vec!["*".to_string()]);
    let max_concurrent_requests = Some(42);

    let http = HttpCtx::new(allowed_hosts, max_concurrent_requests)?;
    http.add_to_linker(&mut linker)?;
    

    linker.module(&mut store, "", &linking)?;

    let run = linker.get(&mut store, "", Some("hello")).unwrap().into_func().unwrap();

    run.typed::<u32,(), _>(&store)?.call(&mut store, 2).unwrap();
    

Ok(())
}
