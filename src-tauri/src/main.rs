#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use analyze::core::{
    export::exporter, filter::filter, split::split, swrite::swrite,
};
use serde_json::value::Value;

#[tauri::command(async)]
fn filter_csv(
    file: PathBuf,
    save_dir: PathBuf,
    remap_csv: PathBuf,
    filter_csv: PathBuf,
) -> Result<Value, String> {
    match filter(file, save_dir, remap_csv, filter_csv) {
        Ok(resp) => Ok(Value::from(resp)),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command(async)]
fn export_csv(
    file: PathBuf,
    save_dir: PathBuf,
    ranges: Vec<(u32, u32)>,
) -> Result<Value, String> {
    match exporter(file, save_dir, ranges) {
        Ok(resp) => Ok(Value::from(resp)),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command(async)]
fn swrite_csv(
    file: PathBuf,
    save_dir: PathBuf,
    ranges_value: String,
    remap_csv: PathBuf,
) -> Result<Value, String> {
    match swrite(file, save_dir, ranges_value, remap_csv) {
        Ok(resp) => Ok(Value::from(resp)),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command(async)]
fn split_csv(
    file: PathBuf,
    save_dir: PathBuf,
    percent: usize,
    remap_csv: PathBuf,
) -> Result<(), String> {
    match split(
        &file,
        &save_dir,
        percent,
        &remap_csv,
        None,
    ) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            filter_csv, export_csv, swrite_csv, split_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
