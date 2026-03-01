use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DokterModel {
    pub kd_dokter: String,
    pub nm_dokter: String,
    pub jk: Option<String>,
    pub tmp_lahir: Option<String>,
    pub tgl_lahir: Option<String>,
    pub gd_darah: Option<String>,
    pub agama: Option<String>,
    pub alamat: Option<String>,
    pub no_tlp: Option<String>,
    pub kd_sps: Option<String>,
    pub nm_sps: Option<String>,
    pub status: Option<String>,
}