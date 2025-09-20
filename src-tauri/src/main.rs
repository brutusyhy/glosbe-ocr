// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use glosbe_ocr_lib::run;

const RUNTIME_MAX_BLOCKING_THREADS: usize = 512;
const RUNTIME_STACK_SIZE: usize = 20 * 1024 * 1024;

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(RUNTIME_MAX_BLOCKING_THREADS)
        .thread_stack_size(RUNTIME_STACK_SIZE)
        .build()
        .unwrap()
        .block_on(async {
            tauri::async_runtime::set(tokio::runtime::Handle::current());
            run().await
        })
}
