use tauri::{AppHandle, Manager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrasiInput {
    pub no_rawat: String,
    pub tgl_registrasi: String,
    pub jam_reg: String,
    pub no_rkm_medis: String,
    pub kd_poli: String,
    pub kd_dokter: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AntrianItem {
    pub nomor: String,
    pub nama: String,
    pub poli: String,
    pub status: String,
}

/// Mendapatkan daftar registrasi
#[tauri::command]
pub async fn get_registrasi_list(
    app: AppHandle,
    tanggal: String,
    kd_poli: Option<String>,
) -> Result<Vec<serde_json::Value>, String> {
    // Placeholder - implement with actual database query
    Ok(vec![])
}

/// Membuat registrasi baru
#[tauri::command]
pub async fn create_registrasi(app: AppHandle, registrasi: RegistrasiInput) -> Result<String, String> {
    // Placeholder - implement with actual database query
    Ok(registrasi.no_rawat)
}

/// Mendapatkan antrian hari ini
#[tauri::command]
pub async fn get_antrian(app: AppHandle, kd_poli: Option<String>) -> Result<Vec<AntrianItem>, String> {
    // Placeholder - return mock data for development
    Ok(vec![
        AntrianItem {
            nomor: "A001".to_string(),
            nama: "Ahmad Suryadi".to_string(),
            poli: "Poli Umum".to_string(),
            status: "Menunggu".to_string(),
        },
        AntrianItem {
            nomor: "A002".to_string(),
            nama: "Siti Nurhaliza".to_string(),
            poli: "Poli Umum".to_string(),
            status: "Dipanggil".to_string(),
        },
    ])
}