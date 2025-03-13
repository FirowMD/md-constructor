mod components;

use crate::components::converter::MarkdownConverter;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use std::fs;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct AppConfig {
    // not implemented yet
    dummy: String,
}

struct AppData {
    markdown_converter: MarkdownConverter,
}

struct Storage {
    app_data: Mutex<AppData>,
}

#[tauri::command]
fn update_markdown_converter(
    markdown_converter: MarkdownConverter,
    app_data: State<Storage>,
) -> String {
    let mut data = app_data.app_data.lock().unwrap();
    data.markdown_converter = markdown_converter.clone();
    markdown_converter.convert()
}

#[tauri::command]
async fn read_json_file(path: String) -> Result<Value, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;
    
    serde_json::from_str(&content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn write_json_file(path: String, content: Value) -> Result<(), String> {
    let json_str = serde_json::to_string_pretty(&content)
        .map_err(|e| e.to_string())?;
    
    fs::write(path, json_str)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn write_text_file(path: String, content: String) -> Result<(), String> {
    fs::write(path, content)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Storage {
            app_data: Mutex::new(AppData {
                markdown_converter: MarkdownConverter::new(String::new(), Vec::new()),
            }),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            update_markdown_converter,
            read_json_file,
            write_json_file,
            write_text_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
