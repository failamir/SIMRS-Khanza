use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliklinikModel {
    pub kd_poli: String,
    pub nm_poli: String,
    pub registrasi: Option<String>,
    pub registrasilama: Option<String>,
    pub status: Option<String>,
}