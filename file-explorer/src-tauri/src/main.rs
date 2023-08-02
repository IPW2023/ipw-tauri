// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env::current_dir;
use std::fs::File;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_current_directory() -> String {
    match current_dir() {
        Ok(directory) => return directory.into_os_string().into_string().unwrap(),
        Err(_error) => panic!("there is no current directory!")
    }
}

#[tauri::command]
fn create_new_file(file_name: String) -> Result<(), String> {
    let try_file = File::create(file_name);
    
    match try_file {
        Ok(_file) => return Ok(()),
        Err(_error) => return Err(String::from("The file couldn't be created!"))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_current_directory, create_new_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
