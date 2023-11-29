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

fn main() {
    // Uncomment to run CLI mode without Tauri
    // let args: Vec<String> = std::env::args().collect();
    // let dirpath = &args[1];

    rexiv2::initialize().expect("Unable to initialize rexiv2");
    rexiv2::set_log_level(rexiv2::LogLevel::ERROR);

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![listdir, rmphoto])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
