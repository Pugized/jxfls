use tauri::Manager;

use crate::logger::Logger;

mod logger;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.get_window("main").unwrap().set_always_on_top(true)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            close,
            logger_warn,
            logger_error,
            logger_fatal,
            logger_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn close(app_handle: tauri::AppHandle, code: i32) {
    app_handle.exit(code);
}

#[tauri::command]
fn logger_warn(scope: String, message: String) {
    Logger::warn(scope, message);
}

#[tauri::command]
fn logger_info(scope: String, message: String) {
    Logger::info(scope, message);
}

#[tauri::command]
fn logger_error(scope: String, message: String) {
    Logger::error(scope, message);
}

#[tauri::command]
fn logger_fatal(scope: String, message: String) {
    Logger::fatal(scope, message);
}
