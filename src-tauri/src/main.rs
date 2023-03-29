// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Runtime;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Message from Rust: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn greet2(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn command_name<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<String, String> {
    println!("Called from {}", window.label());
    Ok("this is worked".into())
    // Ok(())
}

#[derive(Default)]
struct MyState {
    s: std::sync::Mutex<String>,
    t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}
// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn first_cmd(state: tauri::State<'_, MyState>) -> Result<String, String> {
    *state.s.lock().unwrap() = "new string".into();
    state.t.lock().unwrap().insert("key".into(), "value".into());
    // match state.s {
    //     _ => ,
    // }
    Ok("Ok".into())
}

fn main() {
    tauri::Builder::default()
        .manage(MyState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            greet2,
            command_name,
            first_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
