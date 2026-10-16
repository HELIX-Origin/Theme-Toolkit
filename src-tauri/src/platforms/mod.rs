pub mod windows;
pub mod macos;
pub mod linux;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeManifest {
    pub name: String,
    pub display_name: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub wallpaper_path: Option<String>,
    pub sound_scheme_path: Option<String>,
    pub cursor_scheme_path: Option<String>,
    pub accent_color: Option<String>,
    pub dark_mode: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagingResult {
    pub success: bool,
    pub output_path: String,
    pub message: String,
}

#[tauri::command]
pub async fn package_theme(manifest: ThemeManifest) -> Result<PackagingResult, String> {
    #[cfg(target_os = "windows")]
    {
        windows::package_deskthemepack(&manifest).map_err(|e| e.to_string())
    }
    #[cfg(target_os = "macos")]
    {
        macos::package_macos_theme(&manifest).map_err(|e| e.to_string())
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        linux::package_linux_theme(&manifest).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn apply_theme(theme_path: String) -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        windows::apply_deskthemepack(&theme_path).map_err(|e| e.to_string())
    }
    #[cfg(target_os = "macos")]
    {
        macos::apply_macos_theme(&theme_path).map_err(|e| e.to_string())
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        linux::apply_linux_theme(&theme_path).map_err(|e| e.to_string())
    }
}
