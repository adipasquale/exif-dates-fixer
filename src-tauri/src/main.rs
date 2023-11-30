// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use exif_dates_fixer::{Collection, FileInfo};

use std::path::PathBuf;
use tauri::Manager;

#[tauri::command]
fn listdir(dirpath: &str) -> Vec<FileInfo> {
    Collection::new(&PathBuf::from(dirpath)).file_infos
}

#[tauri::command]
fn rmphoto(path: &str) {
    println!("remove path: {}", path);
    std::fs::remove_file(path).expect("Unable to remove file");
}

#[tauri::command]
fn set_date(path: &str, date: &str) -> FileInfo {
    let date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let path = PathBuf::from(path);
    let file_info = exif_dates_fixer::set_date(&path, &date);
    file_info.unwrap()
}

fn main() {
    rexiv2::initialize().expect("Unable to initialize rexiv2");
    rexiv2::set_log_level(rexiv2::LogLevel::ERROR);

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![listdir, rmphoto, set_date])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
