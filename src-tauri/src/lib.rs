pub mod platforms;

use tauri::Manager;

#[tauri::command]
fn get_platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_platform,
            platforms::package_theme,
            platforms::apply_theme
        ])
        .setup(|app| {
            log::info!("Theme Toolkit backend initialized on platform: {}", get_platform());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Theme Toolkit application");
}
