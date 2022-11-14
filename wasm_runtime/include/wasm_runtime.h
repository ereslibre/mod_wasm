/* See doc at: https://github.com/eqrion/cbindgen/blob/master/docs.md#cbindgentoml

/* Generated with cbindgen:0.24.3 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "version.h"

/**
 * Set the root directory for loading Wasm modules.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `path` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `path` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_set_root("/var/www/wasm");
 * ```
 */
void wasm_config_set_root(const char *path);

/**
 * Set the Wasm module filename
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `filename` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `filename` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_set_module("hello.wasm");
 * ```
 */
void wasm_config_set_module(const char *filename);

/**
 * Clears all WASI args for the Wasm module
 */
void wasm_config_clear_args(void);

/**
 * Add a WASI arg for the Wasm module
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `arg` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `arg` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_add_arg("--help");
 * ```
 */
void wasm_config_add_arg(const char *arg);

/**
 * Clears all WASI environment variables for the Wasm module
 */
void wasm_config_clear_envs(void);

/**
 * Set a WASI environment variable for the Wasm module
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `env` and `value` must be valid pointers to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `env` and `value` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, they will trimmed to empty strings.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_add_env("TMP", "/tmp");
 * ```
 */
void wasm_config_add_env(const char *env,
                         const char *value);

/**
 * Clears all WASI preopened dirs for the Wasm module
 */
void wasm_config_clear_dirs(void);

/**
 * Add a WASI preopen dir for the Wasm module
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `dir` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `dir` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_add_dir("/tmp");
 * ```
 */
void wasm_config_add_dir(const char *dir);

/**
 * Clears all WASI propened dirs with mapping for the Wasm module
 */
void wasm_config_clear_mapdirs(void);

/**
 * Add a WASI preopen dir with mapping for the Wasm module
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `map` and `dir` must be valid pointers to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `map` and `dir` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, they will trimmed to empty strings.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_add_mapdir("./", ".");
 * wasm_config_add_mapdir("/wasmhome", "/home/wasm_user");
 * wasm_config_add_mapdir("/wasmlogs", "/var/log");
 * ```
 */
void wasm_config_add_mapdir(const char *map,
                            const char *dir);

/**
 * Set the WASI stdin for the Wasm module
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `filename` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `filename` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_set_module("hello.wasm");
 * ```
 */
void wasm_config_set_stdin(const unsigned char *buffer,
                           uintptr_t size);

/**
 * Initialize the Wasm module
 *
 * Returns empty string if initialization was succesfuly.
 * Otherwise, it returns a string with the error.
 *
 */
const char *wasm_runtime_init_module(void);

/**
 * Run the Wasm module
 *
 * Returns a string with the stdout from the module if execution was succesfuly.
 * Otherwise, trace the error and returns the string explaining the error.
 *
 */
const char *wasm_runtime_run_module(void);

/**
 * Returns raw pointer's ownership
 *
 * After returning a const *char pointer from Rust-world to the C-world, when such a pointer is not going to be used any more,
 * C-world MUST invoke this function in order to Rust-world being able to deallocate the memory.
 * Otherwise, memory will leak.
 *
 */
void return_const_char_ownership(const char *ptr);