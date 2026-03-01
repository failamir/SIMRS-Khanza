use tauri::{AppHandle, Manager};
use crate::database::{get_pool, DashboardStats, StatsRepository, RegistrasiRepository};

/// Mendapatkan statistik untuk dashboard
#[tauri::command]
pub async fn get_dashboard_stats(app: AppHandle) -> Result<DashboardStats, String> {
    let pool = get_pool(&app).ok_or("Database not initialized")?;

    StatsRepository::get_dashboard_stats(&pool)
        .await
        .map_err(|e| e.to_string())
}

/// Mendapatkan daftar pasien terbaru
#[tauri::command]
pub async fn get_recent_patients(app: AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let pool = get_pool(&app).ok_or("Database not initialized")?;

    let registrasi = RegistrasiRepository::get_recent(&pool, 10)
        .await
        .map_err(|e| e.to_string())?;

    // Convert to JSON-friendly format
    Ok(registrasi.into_iter().map(|r| {
        serde_json::json!({
            "noRm": r.no_rkm_medis,
            "nama": r.nm_pasien,
            "tanggal": r.tgl_registrasi,
            "poli": r.nm_poli,
            "status": if r.status == "Sudah" { "aktif" } else { "menunggu" }
        })
    }).collect())
}