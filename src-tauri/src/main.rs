// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use failure::Error;
use tokio::sync::mpsc::channel;

mod filesys;
mod utils;
mod finder;
mod models;

use crate::finder::Finder;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("hello");
    let finder = Finder::new().await?;
    let (sender, rx) = channel(256);

    tokio::spawn(filesys::walk(sender));
    tokio::spawn(filesys::listen(finder.pool.clone(), rx));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
