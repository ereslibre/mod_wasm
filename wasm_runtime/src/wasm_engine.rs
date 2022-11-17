//
// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

// wasm_engine.rs
//
// Using Wasmtime from the Bytecode Alliance as the Wasm Engine
// https://github.com/bytecodealliance/wasmtime

use std::sync::Mutex;

use anyhow::Result;
use once_cell::sync::Lazy; // https://crates.io/crates/once_cell

use wasmtime::{Module, Store, Linker, Instance};
use wasi_common::WasiCtx;

use crate::stdio_buffers::{clear_stdout, read_stdout};
use crate::wasi_context::build_wasi_ctx;

// The following static variables are used to achieve a global, mutable and thread-safe shareable state.
// For that given purpose, it uses [Once Cell](https://crates.io/crates/once_cell).
// Any object will be protected by `once_cell::sync::Lazy` and `std::sync::{Mutex, RwLock}`.

// Lock for Wasm module execution.
// So far, we do not support more than one Wasm invocation simultaneously.
// That would requiere a pool of stdio buffers, and likely a pool of other different Wasmtime objects.
static WASM_EXECUTION_MUTEX: Lazy<Mutex<bool>> = Lazy::new(|| {
    let data = true;
    Mutex::new(data)
});


/// Run the Wasm Module and all the Wasmtime needed objects to later call a function.
///
/// Due to the Wasmtime object's depency graph, a Module and an Engine can be loaded in memory and be safety shared among other entities and threads.
// 
// `run_module()` will create WasiCtx, Linker, Store, Instance and Typed_Function 
// upon the input parameters
// 
// See below the Wasmtime object's dependcy graph:
// Typed_Funct ---> Store, Instance
//       Store ---> Engine, WasiCtx
//    Instance ---> Module, Linker, Store
//      Module ---> Engine
//      Linker ---> Engine
//      Engine ---> 0
//     WasiCtx ---> 0
//
// Steps to load a Wasm module and invoking a function:
// 1) Get a wasmtime::Engine object.
//    Engine::default() will return an Engine with default setup (will be likely good enough).
// 2) A wasmtime::Module can be loaded now into memory for the previous Engine.
//    This is probably them most time-consuming step, so we will execute it at the init stage.
//    Module::from_file(&engine, modulepath)
// 3) Get a wasmtime::Store object. 
//    A new Store requires a reference to the previous Engine and optionally a WASI context (stdio, envs, args, preopen dirs, etc.)
// 4) Get a mutable wasmtime::Linker object.
//    Optionally, add WASI extension to the Linker via wasmtime_wasi::add_to_linker()
// 5) Request the Linker to instantiate the Module for the given Store. That would return an Instance.
//    linker.instantiate(&mut store, &module)
// 6) Obtain the function to invoke from the Instance and passing the Store.
//    instance.get_typed_func::<(), (), _>(&mut *store, "_start")
//
pub fn run_module() -> Result<String> {        
    // this mutex helps to protect from different threads to execute at the same time
    // and clearing stdout to each other before used  
    let mutex = WASM_EXECUTION_MUTEX.lock()
        .expect("ERROR! Poisoned Mutex WASM_EXECUTION_MUTEX on lock()");
    
    clear_stdout();
    invoke_function("_start");
    let output = read_stdout()
        .expect("ERROR! Couldn't read stdout after invoking function!");

    // this drop is redundant, but helps to identify the scope of the mutex
    // and makes explicit use of 'mutex' instead of declaring it as '_mutex'.
    drop(mutex);    

    Ok(output)
}


fn invoke_function(function_name: &str) -> bool {
    // wasmtime shared objects
    let wasmtime_shared_objects = WASMTIME_SHARED_OBJECTS.read()
        .expect("ERROR! Poisoned Mutex WASMTIME_SHARED on write()");
   
    // extract module
    let module = match &wasmtime_shared_objects.module {
        Some(m) => m,
        None => {
            eprintln!("ERROR! Can't build Wasmtime objects becasuse no Wasm module was loaded!");
            return false;
        }
    };

    // build WasiCtx and Store
    let wasi = build_wasi_ctx();
    let mut store: Store<WasiCtx> = Store::new(&wasmtime_shared_objects.engine, wasi);

    // build Linker (with WASI extensions)
    let mut linker: Linker<WasiCtx> = Linker::new(&wasmtime_shared_objects.engine);
    match wasmtime_wasi::add_to_linker(&mut linker, |cx| cx) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("ERROR! Can't add WASI extensions to Wasmtime::Linker! {}", e);
            return false;
        }
    };

    // build Instance
    let instance: Instance = match linker.instantiate(&mut store, module) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("ERROR! Can't instantiate module! {}", e);
            return false;
        }
    };

    // get typed function from instance
    let typed_function = match instance.get_typed_func::<(), (), _>(&mut store, function_name) {
        Ok(tp) => tp,
        Err(e) => {
            eprintln!("ERROR! Can't get typed function '{}' from instance! {}", function_name, e);
            return false;
        }
    };

    // invoke function    
    match typed_function.call(&mut store, ()) {
        Ok(r) => r,
        Err(t) => {
            eprintln!("ERROR! Invocation of function '{}' failed! Wasm Trap returned! {:?}", function_name, t);
            return false;
        }
    };

    true   
}
