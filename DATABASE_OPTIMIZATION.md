# Optimasi Database SIMRS Khanza

## Analisis Detail Masalah

### Total Tabel Saat Ini: 1117

Berdasarkan analisis, ada beberapa kategori masalah:

---

## 1. TABEL DUPLIKAT BER-NOMOR (~20 tabel bisa dihapus)

### Contoh Masalah:

```sql
-- SAAT INI (3 tabel terpisah)
CREATE TABLE antriapotek (loket INT, antrian INT);
CREATE TABLE antriapotek2 (no_resep VARCHAR(14), status ENUM('0','1'), no_rawat VARCHAR(17));
CREATE TABLE antriapotek3 (no_resep VARCHAR(14), status ENUM('0','1'), no_rawat VARCHAR(17));

-- SEHARUSNYA (1 tabel dengan kolom loket)
CREATE TABLE antrian_apotek (
    id INT AUTO_INCREMENT PRIMARY KEY,
    loket INT NOT NULL,
    no_resep VARCHAR(14),
    no_rawat VARCHAR(17),
    status ENUM('0','1') DEFAULT '0',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_loket (loket),
    INDEX idx_status (status),
    INDEX idx_no_rawat (no_rawat)
);
```

### Daftar Tabel Duplikat yang Perlu Digabung:

| Tabel Saat Ini | Usulan Tabel Baru | Kolom Tambahan |
|----------------|-------------------|----------------|
| antriapotek, antriapotek2, antriapotek3 | antrian_apotek | loket |
| antrilabmb, antrilabmb2 | antrian_lab_mb | loket |
| antrilabpa, antrilabpa2 | antrian_lab_pa | loket |
| antrilabpk, antrilabpk2 | antrian_lab_pk | loket |
| antriradiologi, antriradiologi2 | antrian_radiologi | loket |
| set_akun, set_akun2 | set_akun | versi |
| set_akun_ranap, set_akun_ranap2 | set_akun_ranap | versi |
| inacbg_data_terkirim, inacbg_data_terkirim2 | inacbg_data_terkirim | versi |
| inacbg_klaim_baru, inacbg_klaim_baru2 | inacbg_klaim | versi |

---

## 2. TABEL TEMPORARY (~30 tabel bisa dihapus/diubah)

### Masalah:
Tabel-tabel temporary ini seharusnya bukan tabel permanen.

```sql
-- Contoh tabel temporary yang seharusnya tidak permanen
CREATE TABLE temporary_bayar_ralan (...);
CREATE TABLE temporary_payment (...);
CREATE TABLE temporary_resep (...);
```

### Solusi:

**Opsi A: Gunakan VIEW**
```sql
CREATE VIEW view_bayar_ralan AS
SELECT
    r.no_rawat,
    r.tgl_registrasi,
    p.nm_pasien,
    -- kolom lainnya
FROM reg_periksa r
JOIN pasien p ON r.no_rkm_medis = p.no_rkm_medis
WHERE r.status = 'Belum';
```

**Opsi B: Temporary Table di Aplikasi**
```rust
// Di Rust/Tauri
sqlx::query("
    CREATE TEMPORARY TABLE temp_bayar AS
    SELECT * FROM reg_periksa WHERE status = 'Belum'
")
.execute(&pool)
.await?;
```

### Daftar Temporary Tables:
```
temporary, temporary2
temporary_bayar_labkesling
temporary_bayar_ralan
temporary_bayar_ranap
temporary_booking_registrasi
temporary_gizi
temporary_grafik
temporary_lab
temporary_lama_pelayanan_radiologi
temporary_payment
temporary_permintaan_lab
temporary_permintaan_labmb
temporary_permintaan_radiologi
temporary_presensi
temporary_radiologi
temporary_resep
temporary_resume
temporary_sensus_harian
temporary_surveilens_penyakit
temporary_tambahan_potongan
temporary_toko
temppanggilnorawat
temppanggilrm
tampbeli1
tampjual1
tampjurnal
tampjurnal2
tampreturbeli
tampreturjual
tampreturpiutang
tamppiutang
```

---

## 3. TABEL SETTING (~52 tabel bisa jadi 1 tabel)

### Masalah:
52 tabel berbeda untuk menyimpan setting.

### Solusi: Single Settings Table

```sql
-- DROP semua tabel set_*
-- Buat 1 tabel setting terpusat

CREATE TABLE app_settings (
    id INT AUTO_INCREMENT PRIMARY KEY,
    kategori VARCHAR(50) NOT NULL,
    nama_setting VARCHAR(100) NOT NULL,
    nilai TEXT,
    tipe_data ENUM('string', 'int', 'double', 'boolean', 'json') DEFAULT 'string',
    deskripsi TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY uk_kategori_nama (kategori, nama_setting),
    INDEX idx_kategori (kategori)
);

-- Contoh data
INSERT INTO app_settings (kategori, nama_setting, nilai, tipe_data, deskripsi) VALUES
('akun', 'set_akun_default', '112010', 'string', 'Akun default untuk transaksi'),
('akun', 'set_akun_bankbri', '112020', 'string', 'Akun Bank BRI'),
('apotek', 'set_depo_ralan', '1', 'boolean', 'Aktifkan depo rawat jalan'),
('harga', 'set_embalase', '500', 'double', 'Harga embalase'),
('validasi', 'set_validasi_catatan', '1', 'boolean', 'Validasi catatan wajib');
```

---

## 4. TABEL PENILAIAN (~85 tabel bisa jadi ~5 tabel)

### Masalah:
85 tabel penilaian dengan struktur mirip.

### Solusi: Normalisasi dengan Polimorphism

```sql
-- Tabel master penilaian
CREATE TABLE penilaian (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    no_rawat VARCHAR(17) NOT NULL,
    tanggal DATETIME NOT NULL,
    jenis_penilaian VARCHAR(50) NOT NULL,
    nip VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (no_rawat) REFERENCES reg_periksa(no_rawat) ON DELETE CASCADE,
    FOREIGN KEY (nip) REFERENCES petugas(nip) ON DELETE SET NULL,
    INDEX idx_jenis (jenis_penilaian),
    INDEX idx_no_rawat (no_rawat)
);

-- Tabel detail penilaian (flexible)
CREATE TABLE penilaian_detail (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    penilaian_id BIGINT NOT NULL,
    field_name VARCHAR(100) NOT NULL,
    field_value TEXT,
    FOREIGN KEY (penilaian_id) REFERENCES penilaian(id) ON DELETE CASCADE,
    INDEX idx_penilaian (penilaian_id)
);

-- Atau gunakan JSON untuk fleksibilitas
CREATE TABLE penilaian_json (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    no_rawat VARCHAR(17) NOT NULL,
    tanggal DATETIME NOT NULL,
    jenis_penilaian VARCHAR(50) NOT NULL,
    data JSON NOT NULL,  -- Semua field dalam JSON
    nip VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (no_rawat) REFERENCES reg_periksa(no_rawat) ON DELETE CASCADE,
    INDEX idx_jenis (jenis_penilaian)
);

-- Contoh query JSON
SELECT
    no_rawat,
    tanggal,
    jenis_penilaian,
    JSON_EXTRACT(data, '$.tekanan_darah') as tekanan_darah,
    JSON_EXTRACT(data, '$.suhu') as suhu
FROM penilaian_json
WHERE jenis_penilaian = 'awal_keperawatan_ralan';
```

---

## 5. TABEL ANTRIAN TERPUSAT

```sql
-- Buat satu tabel antrian untuk semua lokasi
CREATE TABLE antrian (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    no_rawat VARCHAR(17) NOT NULL,
    lokasi VARCHAR(30) NOT NULL,  -- 'apotek_1', 'apotek_2', 'lab_mb', 'radiologi', etc.
    nomor_antrian INT NOT NULL,
    status ENUM('menunggu', 'dipanggil', 'dilayani', 'selesai', 'batal') DEFAULT 'menunggu',
    prioritas INT DEFAULT 0,  -- untuk prioritas (lansia, difabel, dll)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    called_at TIMESTAMP NULL,
    finished_at TIMESTAMP NULL,
    FOREIGN KEY (no_rawat) REFERENCES reg_periksa(no_rawat) ON DELETE CASCADE,
    UNIQUE KEY uk_lokasi_nomor (lokasi, nomor_antrian),
    INDEX idx_status (status),
    INDEX idx_lokasi (lokasi)
);

-- View untuk antrian aktif
CREATE VIEW v_antrian_aktif AS
SELECT
    a.*,
    r.tgl_registrasi,
    p.nm_pasien,
    p.no_rkm_medis
FROM antrian a
JOIN reg_periksa r ON a.no_rawat = r.no_rawat
JOIN pasien p ON r.no_rkm_medis = p.no_rkm_medis
WHERE a.status IN ('menunggu', 'dipanggil')
ORDER BY a.prioritas DESC, a.nomor_antrian ASC;
```

---

## 6. TABEL SKRINING TERPUSAT

```sql
-- Dari 34 tabel skrining menjadi 2 tabel
CREATE TABLE skrining (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    no_rawat VARCHAR(17) NOT NULL,
    tanggal DATE NOT NULL,
    jenis_skrining VARCHAR(50) NOT NULL,
    hasil ENUM('negatif', 'positif', 'rujukan') NOT NULL,
    skor INT DEFAULT 0,
    data JSON,  -- detail hasil skrining
    nip VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (no_rawat) REFERENCES reg_periksa(no_rawat) ON DELETE CASCADE,
    INDEX idx_jenis (jenis_skrining)
);

-- Jenis skrining:
-- 'adiksi_nikotin', 'anemia', 'diabetes_melitus', 'gizi',
-- 'hipertensi', 'tbc', 'thalessemia', 'kanker_kolorektal', etc.
```

---

## 7. TABEL SURAT TERPUSAT

```sql
-- Dari 45 tabel surat menjadi 2 tabel
CREATE TABLE surat (
    id INT AUTO_INCREMENT PRIMARY KEY,
    no_surat VARCHAR(20) NOT NULL UNIQUE,
    jenis_surat VARCHAR(50) NOT NULL,
    no_rawat VARCHAR(17),
    no_rkm_medis VARCHAR(15),
    tanggal DATE NOT NULL,
    konten JSON NOT NULL,  -- isi surat dalam format JSON
    nip_pembuat VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (no_rawat) REFERENCES reg_periksa(no_rawat) ON DELETE SET NULL,
    INDEX idx_jenis (jenis_surat),
    INDEX idx_tanggal (tanggal)
);

CREATE TABLE surat_template (
    id INT AUTO_INCREMENT PRIMARY KEY,
    jenis_surat VARCHAR(50) NOT NULL UNIQUE,
    nama_template VARCHAR(100) NOT NULL,
    template TEXT NOT NULL,  -- HTML template
    fields JSON NOT NULL,  -- daftar field yang diperlukan
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

---

## RINGKASAN OPTIMASI

| Kategori | Jumlah Saat Ini | Usulan Jumlah | Pengurangan |
|----------|-----------------|---------------|-------------|
| Tabel Duplikat | ~20 | ~5 | 15 |
| Temporary Tables | ~30 | 0 (view/app) | 30 |
| Setting Tables | ~52 | 1 | 51 |
| Penilaian Tables | ~85 | ~5 | 80 |
| Antrian Tables | ~30 | 1 | 29 |
| Skrining Tables | ~34 | 1 | 33 |
| Surat Tables | ~45 | 2 | 43 |
| **TOTAL** | **~300** | **~15** | **~285** |

**Estimasi Total Pengurangan: Dari 1117 tabel menjadi ~300-400 tabel**

---

## SQL MIGRATION SCRIPT OUTLINE

```sql
-- Migration: 001_optimize_database.sql

-- Step 1: Backup data
CREATE TABLE backup_log (
    id INT AUTO_INCREMENT PRIMARY KEY,
    table_name VARCHAR(100),
    rows_affected INT,
    backup_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Step 2: Create new consolidated tables
-- (lihat detail di atas)

-- Step 3: Migrate data
-- INSERT INTO new_table SELECT ... FROM old_tables;

-- Step 4: Verify data integrity
-- (queries untuk verifikasi)

-- Step 5: Drop old tables (setelah verifikasi)
-- DROP TABLE old_table1, old_table2, ...;

-- Step 6: Update foreign keys and indexes
```

---

*Document ini berisi detail teknis untuk optimasi database*