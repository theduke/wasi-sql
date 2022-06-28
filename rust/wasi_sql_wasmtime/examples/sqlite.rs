use wasi_sql_wasmtime::sqlite::State;

struct StateWithWasi {
    sqlite: State,
    wasi: wasmtime_wasi::WasiCtx,
}

fn main() {
    let engine = wasmtime::Engine::default();
    let wasm = include_bytes!("../../target/wasm32-wasi/debug/examples/sqlite.wasm");

    let module = wasmtime::Module::new(&engine, &wasm).unwrap();

    let mut linker = wasmtime::Linker::<StateWithWasi>::new(&engine);

    wasmtime_wasi::sync::add_to_linker(&mut linker, |s| &mut s.wasi).unwrap();
    State::add_to_linker(&mut linker, |s| &mut s.sqlite);

    let state = State::new();

    let wasi = wasmtime_wasi::sync::WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();

    let full = StateWithWasi {
        sqlite: state,
        wasi,
    };

    let mut store = wasmtime::Store::new(&engine, full);

    let instance = linker.instantiate(&mut store, &module).unwrap();

    let main = instance.get_func(&mut store, "main").unwrap();
    let mut out: [wasmtime::Val; 1] = [0i32.into()];
    main.call(&mut store, &[0i32.into(), 0i32.into()], &mut out)
        .unwrap();
}
