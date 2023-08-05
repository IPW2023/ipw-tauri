// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env::current_dir;
use std::fs::{File, write, read_dir, read_to_string};

#[tauri::command]
fn example_command() {
    println!("I like pizza");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            example_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
