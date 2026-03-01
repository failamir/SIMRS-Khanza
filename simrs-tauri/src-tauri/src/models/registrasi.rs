use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrasiModel {
    pub no_rawat: String,
    pub tgl_registrasi: String,
    pub jam_reg: String,
    pub no_rkm_medis: String,
    pub kd_poli: String,
    pub kd_dokter: String,
    pub kd_pj: String,
    pub stts: String,
    pub status: String,
    pub kd_dokter_bpjs: Option<String>,
}