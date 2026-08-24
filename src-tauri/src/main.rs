#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Emitter, Manager};

#[derive(Default, Serialize, Deserialize)]
struct InstallerState {
    current_step: u32,
    install_path: String,
    components: Vec<String>,
}

struct AppState {
    state: Mutex<InstallerState>,
}

#[derive(Clone, Serialize)]
struct DownloadProgress {
    phase: String,
    progress: f64,
    message: String,
}

#[derive(Deserialize)]
struct GitHubRelease {
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
    size: u64,
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

fn get_asset_name() -> String {
    if cfg!(target_os = "windows") {
        "WayOfTeams".to_string()
    } else if cfg!(target_os = "macos") {
        "WayOfTeams".to_string()
    } else {
        "WayOfTeams".to_string()
    }
}

#[tauri::command]
async fn download_and_install(
    app: tauri::AppHandle,
    install_path: String,
    components: Vec<String>,
) -> Result<String, String> {
    let state = app.state::<AppState>();
    {
        let mut s = state.state.lock().unwrap();
        s.install_path = install_path.clone();
        s.components = components.clone();
    }

    // Phase 1: Fetch release info from GitHub
    app.emit("install-progress", DownloadProgress {
        phase: "download".to_string(),
        progress: 0.0,
        message: "Fetching release information...".to_string(),
    }).ok();

    let client = reqwest::Client::new();
    let release_url = "https://api.github.com/repos/Way-Of/wayofteams-releases/releases/latest";
    
    let release: GitHubRelease = client
        .get(release_url)
        .header("User-Agent", "WayOfTeams-Installer")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch release: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse release JSON: {}", e))?;

    // Find platform-specific asset
    let asset_name = get_asset_name();
    let asset = release.assets.iter().find(|a| a.name.contains(&asset_name))
        .ok_or_else(|| format!("No asset found matching '{}'", asset_name))?;

    let download_url = asset.browser_download_url.clone();
    let file_size = asset.size;

    // Phase 2: Download
    app.emit("install-progress", DownloadProgress {
        phase: "download".to_string(),
        progress: 0.0,
        message: format!("Downloading {}...", asset.name),
    }).ok();

    let response = client
        .get(&download_url)
        .header("User-Agent", "WayOfTeams-Installer")
        .send()
        .await
        .map_err(|e| format!("Failed to start download: {}", e))?;

    let total_size = response.content_length().unwrap_or(file_size);
    let mut downloaded: u64 = 0;

    let install_dir = PathBuf::from(&install_path);
    std::fs::create_dir_all(&install_dir)
        .map_err(|e| format!("Failed to create install directory: {}", e))?;

    let archive_path = install_dir.join(&asset.name);

    // Download with progress
    let mut file = std::fs::File::create(&archive_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;

    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Download error: {}", e))?;
        file.write_all(&chunk).map_err(|e| format!("Write error: {}", e))?;
        downloaded += chunk.len() as u64;

        let progress = if total_size > 0 {
            (downloaded as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };

        app.emit("install-progress", DownloadProgress {
            phase: "download".to_string(),
            progress,
            message: format!("Downloading... {}/{} MB", downloaded / 1_048_576, total_size / 1_048_576),
        }).ok();
    }

    drop(file);

    // Phase 3: Extract
    app.emit("install-progress", DownloadProgress {
        phase: "extract".to_string(),
        progress: 0.0,
        message: "Extracting files...".to_string(),
    }).ok();

    let file = std::fs::File::open(&archive_path)
        .map_err(|e| format!("Failed to open archive: {}", e))?;

    if archive_path.extension().map_or(false, |ext| ext == "zip") {
        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

        let total_files = archive.len() as f64;
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)
                .map_err(|e| format!("Failed to read ZIP entry: {}", e))?;

            let outpath = install_dir.join(entry.mangled_name());

            if entry.is_dir() {
                std::fs::create_dir_all(&outpath)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            } else {
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }
                let mut outfile = std::fs::File::create(&outpath)
                    .map_err(|e| format!("Failed to create file: {}", e))?;
                std::io::copy(&mut entry, &mut outfile)
                    .map_err(|e| format!("Failed to extract file: {}", e))?;
            }

            let progress = ((i as f64 + 1.0) / total_files) * 100.0;
            app.emit("install-progress", DownloadProgress {
                phase: "extract".to_string(),
                progress,
                message: format!("Extracting {}...", entry.name()),
            }).ok();
        }
    } else if archive_path.extension().map_or(false, |ext| ext == "tar" || archive_path.to_string_lossy().ends_with(".tar.gz")) {
        // Handle tar.gz - use tar crate
        return Err("tar.gz extraction not yet implemented".to_string());
    }

    // Phase 4: Install (create shortcuts, configure)
    app.emit("install-progress", DownloadProgress {
        phase: "install".to_string(),
        progress: 0.0,
        message: "Finalizing installation...".to_string(),
    }).ok();

    // Clean up archive
    let _ = std::fs::remove_file(&archive_path);

    app.emit("install-progress", DownloadProgress {
        phase: "install".to_string(),
        progress: 100.0,
        message: "Installation complete!".to_string(),
    }).ok();

    Ok(format!("WayOfTeams installed to {}", install_path))
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
