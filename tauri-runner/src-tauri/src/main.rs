#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod pkg_json_utils;
mod scripts_exec_utils;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct RunnerState {
    projects: Arc<Mutex<Vec<String>>>,
    child_processes: Arc<Mutex<HashMap<String, String>>>
}

#[tauri::command]
fn get_commands(state: tauri::State<RunnerState>) -> Vec<String> {
    let projects = state.projects.lock().unwrap();
    pkg_json_utils::extract_scripts(projects.to_vec())
}

#[tauri::command]
fn exec_command(
    command: &str,
    state: tauri::State<RunnerState>
) -> String {
    let mut active_processes = state.child_processes.lock().unwrap();
    scripts_exec_utils::terminate_all(&active_processes);

    let projects = state.projects.lock().unwrap();
    if let Some(ids) = scripts_exec_utils::exec_scripts(&command, projects.to_vec()) {
        // *state.child_processes.lock().unwrap() = ids;
    };
    "Running".to_string()
}

#[tauri::command]
fn set_runnable_project(path: &str, state: tauri::State<RunnerState>) -> String {
    if !pkg_json_utils::is_valid_path(&path) {
        "404".to_string()
    } else {
        let mut projects = state.projects.lock().unwrap();
        projects.push(path.to_string());
        "OK".to_string()
    }
}

fn main() {
    let state = RunnerState {
        projects: Default::default(),
        child_processes: Default::default()
    };
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![set_runnable_project, get_commands, exec_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
