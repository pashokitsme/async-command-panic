use std::time::Duration;

use tauri::*;

#[command]
fn sleep() {
    std::thread::sleep(Duration::from_secs(1));
    println!("wait done");
}

#[command(async)]
fn sleep_async() {
    std::thread::sleep(Duration::from_secs(1));
    println!("wait done");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![sleep, sleep_async])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
