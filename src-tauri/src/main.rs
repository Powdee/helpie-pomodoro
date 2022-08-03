#![cfg_attr(
    all(not(debug_assertions), target_os = "window&s"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
struct PomodoroData {
    created: String,
    started: Vec<i32>,
    stopped: Option<Vec<i32>>,
}

fn deserialize_json(state: &String) -> Result<()> {
    let p: PomodoroData = serde_json::from_str(&state)?;

    println!("First round was created at {}", p.created);
    println!("First round was started at {}", p.started[0]);

    return Ok(());
}

#[tauri::command]
fn gather_history_data(state: String) -> String {
    match deserialize_json(&state) {
        Err(e) => println!("{:?}", e),
        _ => println!("Success"),
    }

    return format!("Created smth special");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![gather_history_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
