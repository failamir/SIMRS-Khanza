use sqlx::{FromRow, MySql, Pool, query_as};
use serde::{Deserialize, Serialize};

pub type DbPool = Pool<MySql>;

/// Model untuk data pasien
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Pasien {
    pub no_rkm_medis: String,
    pub nm_pasien: String,
    pub no_ktp: Option<String>,
    pub jk: String,
    pub tgl_lahir: Option<String>,
    pub alamat: Option<String>,
    pub no_tlp: Option<String>,
}

/// Model untuk registrasi
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Registrasi {
    pub no_rawat: String,
    pub tgl_registrasi: String,
    pub jam_reg: String,
    pub no_rkm_medis: String,
    pub nm_pasien: String,
    pub kd_poli: String,
    pub nm_poli: String,
    pub kd_dokter: String,
    pub nm_dokter: String,
    pub status: String,
}

/// Model untuk statistik dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_pasien: i64,
    pub pasien_hari_ini: i64,
    pub rawat_jalan: i64,
    pub rawat_inap: i64,
}

/// Repository untuk operasi database pasien
pub struct PasienRepository;

impl PasienRepository {
    pub async fn find_all(pool: &DbPool, limit: i64, offset: i64) -> Result<Vec<Pasien>, sqlx::Error> {
        sqlx::query_as::<_, Pasien>(
            "SELECT no_rkm_medis, nm_pasien, no_ktp, jk, tgl_lahir, alamat, no_tlp
             FROM pasien
             ORDER BY no_rkm_medis DESC
             LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &DbPool, no_rkm_medis: &str) -> Result<Option<Pasien>, sqlx::Error> {
        sqlx::query_as::<_, Pasien>(
            "SELECT no_rkm_medis, nm_pasien, no_ktp, jk, tgl_lahir, alamat, no_tlp
             FROM pasien WHERE no_rkm_medis = ?"
        )
        .bind(no_rkm_medis)
        .fetch_optional(pool)
        .await
    }

    pub async fn search(pool: &DbPool, keyword: &str) -> Result<Vec<Pasien>, sqlx::Error> {
        let pattern = format!("%{}%", keyword);
        sqlx::query_as::<_, Pasien>(
            "SELECT no_rkm_medis, nm_pasien, no_ktp, jk, tgl_lahir, alamat, no_tlp
             FROM pasien
             WHERE nm_pasien LIKE ? OR no_rkm_medis LIKE ? OR no_ktp LIKE ?
             ORDER BY nm_pasien
             LIMIT 50"
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(pool)
        .await
    }

    pub async fn count(pool: &DbPool) -> Result<i64, sqlx::Error> {
        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pasien")
            .fetch_one(pool)
            .await?;
        Ok(result.0)
    }
}

/// Repository untuk statistik
pub struct StatsRepository;

impl StatsRepository {
    pub async fn get_dashboard_stats(pool: &DbPool) -> Result<DashboardStats, sqlx::Error> {
        // Get total pasien
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pasien")
            .fetch_one(pool)
            .await?;

        // Get pasien hari ini (from reg_periksa)
        let hari_ini: (i64,) = sqlx::query_as(
            "SELECT COUNT(DISTINCT no_rkm_medis) FROM reg_periksa WHERE tgl_registrasi = CURDATE()"
        )
        .fetch_one(pool)
        .await?;

        // Get rawat jalan hari ini
        let rawat_jalan: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM reg_periksa WHERE tgl_registrasi = CURDATE() AND status != 'Batal'"
        )
        .fetch_one(pool)
        .await?;

        // Get rawat inap
        let rawat_inap: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM kamar_inap WHERE tgl_keluar IS NULL OR tgl_keluar = '0000-00-00'"
        )
        .fetch_one(pool)
        .await?;

        Ok(DashboardStats {
            total_pasien: total.0,
            pasien_hari_ini: hari_ini.0,
            rawat_jalan: rawat_jalan.0,
            rawat_inap: rawat_inap.0,
        })
    }
}

/// Repository untuk registrasi
pub struct RegistrasiRepository;

impl RegistrasiRepository {
    pub async fn get_recent(pool: &DbPool, limit: i64) -> Result<Vec<Registrasi>, sqlx::Error> {
        sqlx::query_as(
            r#"SELECT
                r.no_rawat,
                r.tgl_registrasi,
                r.jam_reg,
                r.no_rkm_medis,
                p.nm_pasien,
                r.kd_poli,
                pk.nm_poli,
                r.kd_dokter,
                d.nm_dokter,
                r.status
               FROM reg_periksa r
               JOIN pasien p ON r.no_rkm_medis = p.no_rkm_medis
               JOIN poliklinik pk ON r.kd_poli = pk.kd_poli
               JOIN dokter d ON r.kd_dokter = d.kd_dokter
               ORDER BY r.tgl_registrasi DESC, r.jam_reg DESC
               LIMIT ?"#
        )
        .bind(limit)
        .fetch_all(pool)
        .await
    }
}