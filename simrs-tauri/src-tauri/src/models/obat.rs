use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObatModel {
    pub kode_brng: String,
    pub nama_brng: String,
    pub kode_sat: Option<String>,
    pub kode_kategori: Option<String>,
    pub kode_golongan: Option<String>,
    pub harga_beli: Option<f64>,
    pub harga_ralan: Option<f64>,
    pub harga_ranap: Option<f64>,
    pub stok: Option<f64>,
    pub status: Option<String>,
}