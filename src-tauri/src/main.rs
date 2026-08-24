#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::Manager;

#[derive(Default, Serialize, Deserialize)]
struct InstallerState {
    current_step: u32,
    install_path: String,
    components: Vec<String>,
    download_progress: f64,
    extract_progress: f64,
    install_progress: f64,
}

struct AppState {
    state: Mutex<InstallerState>,
}

#[tauri::command]
fn get_os() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else {
        "linux".to_string()
    }
}

#[tauri::command]
fn get_default_path() -> String {
    if cfg!(target_os = "windows") {
        "C:\\Program Files\\WayOfTeams".to_string()
    } else if cfg!(target_os = "macos") {
        "/Applications/WayOfTeams".to_string()
    } else {
        "/opt/wayofteams".to_string()
    }
}

#[tauri::command]
fn get_release_url() -> String {
    "https://api.github.com/repos/Way-Of/wayofteams-releases/releases/latest".to_string()
}

#[tauri::command]
async fn download_and_install(
    state: tauri::State<'_, AppState>,
    install_path: String,
    components: Vec<String>,
) -> Result<String, String> {
    // TODO: Implement actual download from GitHub Releases
    // 1. Fetch latest release JSON from GitHub API
    // 2. Download the platform-specific archive
    // 3. Extract to install_path
    // 4. Create shortcuts
    // 5. Return success message

    Ok(format!(
        "Installation complete at {} with components: {:?}",
        install_path, components
    ))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            state: Mutex::new(InstallerState::default()),
        })
        .invoke_handler(tauri::generate_handler![
            get_os,
            get_default_path,
            get_release_url,
            download_and_install,
        ])
        .run(tauri::generate_context!())
        .expect("error while running installer");
}
