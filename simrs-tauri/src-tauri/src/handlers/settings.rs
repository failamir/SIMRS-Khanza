use tauri::{AppHandle, Manager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub kategori: String,
    pub nama: String,
    pub nilai: String,
}

/// Mendapatkan semua pengaturan
#[tauri::command]
pub async fn get_settings(app: AppHandle, kategori: Option<String>) -> Result<Vec<Setting>, String> {
    // Placeholder - implement with actual database query
    Ok(vec![
        Setting {
            kategori: "aplikasi".to_string(),
            nama: "nama_instansi".to_string(),
            nilai: "Rumah Sakit Khanza".to_string(),
        },
        Setting {
            kategori: "aplikasi".to_string(),
            nama: "alamat".to_string(),
            nilai: "Jl. Contoh No. 123".to_string(),
        },
    ])
}

/// Mengupdate pengaturan
#[tauri::command]
pub async fn update_setting(app: AppHandle, setting: Setting) -> Result<(), String> {
    // Placeholder - implement with actual database query
    Ok(())
}