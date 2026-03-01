use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// Model data pasien
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasienModel {
    pub no_rkm_medis: String,
    pub nm_pasien: String,
    pub no_ktp: Option<String>,
    pub jk: String,
    pub tmp_lahir: Option<String>,
    pub tgl_lahir: Option<String>,
    pub alamat: Option<String>,
    pub gd_darah: Option<String>,
    pub pekerjaan: Option<String>,
    pub stts_nikah: Option<String>,
    pub agama: Option<String>,
    pub no_tlp: Option<String>,
    pub umur: Option<String>,
    pub pendidikan: Option<String>,
    pub keluarga: Option<String>,
}

/// Input untuk membuat pasien baru
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePasienInput {
    pub nm_pasien: String,
    pub no_ktp: Option<String>,
    pub jk: String,
    pub tmp_lahir: Option<String>,
    pub tgl_lahir: Option<String>,
    pub alamat: Option<String>,
    pub gd_darah: Option<String>,
    pub pekerjaan: Option<String>,
    pub stts_nikah: Option<String>,
    pub agama: Option<String>,
    pub no_tlp: Option<String>,
    pub pendidikan: Option<String>,
    pub keluarga: Option<String>,
}

/// Input untuk update pasien
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasienInput {
    pub no_rkm_medis: String,
    pub nm_pasien: Option<String>,
    pub no_ktp: Option<String>,
    pub jk: Option<String>,
    pub tmp_lahir: Option<String>,
    pub tgl_lahir: Option<String>,
    pub alamat: Option<String>,
    pub gd_darah: Option<String>,
    pub pekerjaan: Option<String>,
    pub stts_nikah: Option<String>,
    pub agama: Option<String>,
    pub no_tlp: Option<String>,
    pub pendidikan: Option<String>,
    pub keluarga: Option<String>,
}