use std::fmt::format;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::path::{ Path, PathBuf };

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_dir(path: String) -> Result<Vec<String>, String> {
    let dir: &Path = Path::new(&path);
    let rd = fs::read_dir(&dir).map_err(|e| format!("read_dir error: {}", e))?;
    let mut entries = Vec::new();
    for entry in rd {
        let entry = entry.map_err(|e| format!("dir entry error: {}", e))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        entries.push(name.trim().to_string());
    }
    Ok(entries)
}

#[tauri::command]
fn parse_csv(path: String) -> Result<Vec<Vec<String>>, String> {
    let mut rdr = csv::Reader::from_path(path).map_err(|e| format!("csv open error: {}", e))?;
    let mut rows: Vec<Vec<String>> = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|e| format!("csv record error: {}", e))?;
        let row = record
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        rows.push(row);
    }
    Ok(rows)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_dir, parse_csv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
