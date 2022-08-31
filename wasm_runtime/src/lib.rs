//! **wasm_runtime** is a Rust library intended to:
//! * Integrate with Wasm engines (such as [Wasmtime](https://github.com/bytecodealliance/wasmtime)). 
//! * Provide a thin C API for instantiating, running, and managing Wasm modules.

use std::sync::RwLock;
use once_cell::sync::Lazy; // https://crates.io/crates/once_cell

// modules
mod wasmengine;
mod ffi_utils;
mod c_api;

// The following static variables are used to achieve a global, mutable and thread-safe shareable state.
// For that given purpose, it uses [Once Cell](https://crates.io/crates/once_cell).
// Any object will be protected by `once_cell::sync::Lazy` and `std::sync::Mutex`.
//
//

// Stores the root directory for loading Wasm modules.
static WASM_RUNTIME_CONFIG_ROOT: Lazy<RwLock<String>> = Lazy::new(|| {
    let data = String::new();
    RwLock::new(data)
});

// Stores the Wasm module filename.
static WASM_RUNTIME_CONFIG_MODULE: Lazy<RwLock<String>> = Lazy::new(|| {
    let data = String::new();
    RwLock::new(data)
});

// Stores the WASI args for the Wasm module.
static WASM_RUNTIME_CONFIG_WASI_ARGS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| {
    let data: Vec<String> = Vec::new();
    RwLock::new(data)
});

// Stores the WASI env variables for the Wasm module.
static WASM_RUNTIME_CONFIG_WASI_ENVS: Lazy<RwLock<Vec<(String, String)>>> = Lazy::new(|| {
    let data: Vec<(String, String)> = Vec::new();
    RwLock::new(data)
});

// Stores the WASI preopen dirs for the Wasm module.
static WASM_RUNTIME_CONFIG_WASI_DIRS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| {
    let data: Vec<String> = Vec::new();
    RwLock::new(data)
});

// Stores the WASI preopen dirs with mapping for the Wasm module.
static WASM_RUNTIME_CONFIG_WASI_MAPDIRS: Lazy<RwLock<Vec<(String, String)>>> = Lazy::new(|| {
    let data: Vec<(String, String)> = Vec::new();
    RwLock::new(data)
});


