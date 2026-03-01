use tauri::{AppHandle, Manager};
use serde::{Deserialize, Serialize};
use crate::database::{get_pool, Pasien, PasienRepository};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasienInput {
    pub no_rkm_medis: String,
    pub nm_pasien: String,
    pub no_ktp: Option<String>,
    pub jk: String,
    pub tgl_lahir: Option<String>,
    pub alamat: Option<String>,
    pub no_tlp: Option<String>,
}

/// Mendapatkan daftar pasien dengan pagination
#[tauri::command]
pub async fn get_pasien_list(app: AppHandle, page: i64, limit: i64) -> Result<Vec<Pasien>, String> {
    let pool = get_pool(&app).ok_or("Database not initialized")?;
    let offset = (page - 1) * limit;

    PasienRepository::find_all(&pool, limit, offset)
        .await
        .map_err(|e| e.to_string())
}

/// Mendapatkan detail pasien berdasarkan nomor rekam medis
#[tauri::command]
pub async fn get_pasien_by_id(app: AppHandle, no_rkm_medis: String) -> Result<Option<Pasien>, String> {
    let pool = get_pool(&app).ok_or("Database not initialized")?;

    PasienRepository::find_by_id(&pool, &no_rkm_medis)
        .await
        .map_err(|e| e.to_string())
}

/// Membuat pasien baru
#[tauri::command]
pub async fn create_pasien(app: AppHandle, pasien: PasienInput) -> Result<Pasien, String> {
    let pool = get_pool(&app).ok_or("Database not initialized")?;

    sqlx::query(
        r#"INSERT INTO pasien (no_rkm_medis, nm_pasien, no_ktp, jk, tgl_lahir, alamat, no_tlp)
           VALUES (?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&pasien.no_rkm_medis)
    .bind(&pasien.nm_pasien)
    .bind(&pasien.no_ktp)
    .bind(&pasien.jk)
    .bind(&pasien.tgl_lahir)
    .bind(&pasien.alamat)
    .bind(&pasien.no_tlp)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Return the created patient
    PasienRepository::find_by_id(&pool, &pasien.no_rkm_medis)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Failed to retrieve created patient".to_string())
}

/// Mengupdate data pasien
#[tauri::command]
pub async fn update_pasien(app: AppHandle, pasien: PasienInput) -> Result<Pasien, String> {
    let pool = get_pool(&app).ok_or("Database not initialized")?;

    sqlx::query(
        r#"UPDATE pasien SET
           nm_pasien = ?, no_ktp = ?, jk = ?, tgl_lahir = ?, alamat = ?, no_tlp = ?
           WHERE no_rkm_medis = ?"#
    )
    .bind(&pasien.nm_pasien)
    .bind(&pasien.no_ktp)
    .bind(&pasien.jk)
    .bind(&pasien.tgl_lahir)
    .bind(&pasien.alamat)
    .bind(&pasien.no_tlp)
    .bind(&pasien.no_rkm_medis)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    PasienRepository::find_by_id(&pool, &pasien.no_rkm_medis)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Failed to retrieve updated patient".to_string())
}

/// Menghapus pasien
#[tauri::command]
pub async fn delete_pasien(app: AppHandle, no_rkm_medis: String) -> Result<(), String> {
    let pool = get_pool(&app).ok_or("Database not initialized")?;

    sqlx::query("DELETE FROM pasien WHERE no_rkm_medis = ?")
        .bind(&no_rkm_medis)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Mencari pasien berdasarkan keyword
#[tauri::command]
pub async fn search_pasien(app: AppHandle, keyword: String) -> Result<Vec<Pasien>, String> {
    let pool = get_pool(&app).ok_or("Database not initialized")?;

    PasienRepository::search(&pool, &keyword)
        .await
        .map_err(|e| e.to_string())
}