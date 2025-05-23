// src-tauri/src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: u64,
    title: String,
    completed: bool,
    list: String,
    description: Option<String>,
    due_date: Option<String>,
    repeat: Option<String>,
    attachments: Option<Vec<String>>,
}

const FILE_NAME: &str = "tasks.json";

fn get_data_path() -> PathBuf {
    let base = dirs::data_local_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
    let full_path = base.join("todo-burke").join(FILE_NAME);
    println!("[get_data_path] path: {:?}", full_path);
    full_path
}

#[tauri::command]
fn load_tasks() -> Vec<Task> {
    let path = get_data_path();
    println!("[load_tasks] loading from: {:?}", path);
    if let Ok(mut file) = File::open(&path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            match serde_json::from_str(&contents) {
                Ok(tasks) => tasks,
                Err(err) => {
                    println!("[load_tasks] failed to parse JSON: {}", err);
                    vec![]
                }
            }
        } else {
            println!("[load_tasks] failed to read file contents");
            vec![]
        }
    } else {
        println!("[load_tasks] file not found");
        vec![]
    }
}

#[tauri::command]
fn save_tasks(tasks: Vec<Task>) -> Result<(), String> {
    let path = get_data_path();
    println!("[save_tasks] saving to: {:?}", path);
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            println!("[save_tasks] failed to create directory: {}", e);
            return Err(e.to_string());
        }
    }
    let data = serde_json::to_string_pretty(&tasks).map_err(|e| e.to_string())?;
    match File::create(&path) {
        Ok(mut file) => match file.write_all(data.as_bytes()) {
            Ok(_) => {
                println!("[save_tasks] successfully wrote tasks");
                Ok(())
            }
            Err(e) => {
                println!("[save_tasks] failed to write data: {}", e);
                Err(e.to_string())
            }
        },
        Err(e) => {
            println!("[save_tasks] failed to create file: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn load_lists() -> Vec<String> {
    let tasks = load_tasks();
    let unique: HashSet<String> = tasks.into_iter().map(|t| t.list).collect();
    let mut lists: Vec<String> = unique.into_iter().collect();
    lists.sort();
    lists
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_tasks, save_tasks, load_lists])
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
