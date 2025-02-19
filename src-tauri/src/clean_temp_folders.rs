use std::env;
use std::fs;
use std::path::PathBuf;
use tauri::command;

#[command]
fn clean_temp_folders() -> Result<String, String> {
    let temp_dirs = vec![
        env::var("TEMP").unwrap_or_else(|_| "C:\\Windows\\Temp".to_string()),
        env::var("TMP").unwrap_or_else(|_| "C:\\Windows\\Temp".to_string()),
    ];

    for temp_dir in &temp_dirs {
        let path = PathBuf::from(temp_dir);
        if path.exists() {
            match fs::read_dir(&path) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        let entry_path = entry.path();
                        if entry_path.is_file() {
                            let _ = fs::remove_file(&entry_path);
                        } else if entry_path.is_dir() {
                            let _ = fs::remove_dir_all(&entry_path);
                        }
                    }
                }
                Err(err) => {
                    return Err(format!("Failed to read temp folder {}: {}", temp_dir, err))
                }
            }
        }
    }
    Ok("Temporary folders cleaned successfully".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![clean_temp_folders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
