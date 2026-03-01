# Panduan AI: SIMRS Khanza -> Tauri Conversion

## Ringkasan Proyek
Program SIMRS Khanza adalah sistem informasi manajemen rumah sakit berbasis Java (desktop) yang akan dikonversi ke Tauri (Rust + Web Frontend).

---

## Analisis Database (sik.sql)

### Statistik Database
- **Total Tabel: 1117 tabel** (sangat banyak, perlu optimasi)
- **Database Engine:** MariaDB 10.4.28
- **Charset:** latin1_swedish_ci (perlu upgrade ke utf8mb4)

---

## Kategori Tabel & Optimasi yang Disarankan

### 1. TABEL DUPLIKAT BER-NOMOR (Perlu Digabung)
Tabel-tabel ini seharusnya digabung menjadi 1 tabel dengan kolom `loket` atau `versi`:

```
ANTRIAN APOTEK:
- antriapotek, antriapotek2, antriapotek3
  -> GABUNG JADI: antrian_apotek (tambah kolom: loket INT)

ANTRIAN LAB:
- antrilabmb, antrilabmb2
- antrilabpa, antrilabpa2
- antrilabpk, antrilabpk2
  -> GABUNG JADI: antrian_lab (tambah kolom: jenis_lab ENUM('mb','pa','pk'), loket INT)

ANTRIAN RADIOLOGI:
- antriradiologi, antriradiologi2
  -> GABUNG JADI: antrian_radiologi (tambah kolom: loket INT)

DATA TRIASE:
- data_triase_igddetail_skala1 s/d skala5
- master_triase_skala1 s/d skala5
  -> GABUNG JADI: data_triase_detail (tambah kolom: skala INT)

INACBG:
- inacbg_data_terkirim, inacbg_data_terkirim2
- inacbg_grouping_stage1, inacbg_grouping_stage12
- inacbg_klaim_baru, inacbg_klaim_baru2
  -> Evaluasi: apakah benar-benar berbeda atau bisa digabung

SET AKUN:
- set_akun, set_akun2
- set_akun_ranap, set_akun_ranap2
  -> GABUNG JADI: set_akun (tambah kolom: versi INT)
```

**Estimasi Pengurangan: ~20 tabel**

---

### 2. TABEL TEMPORARY (Perlu Dihapus/Diubah)
Tabel-tabel ini seharusnya bukan tabel permanen, tapi view atau temporary table:

```
TEMPORARY TABLES (21 tabel):
- temporary, temporary2
- temporary_bayar_labkesling
- temporary_bayar_ralan
- temporary_bayar_ranap
- temporary_booking_registrasi
- temporary_gizi
- temporary_grafik
- temporary_lab
- temporary_lama_pelayanan_radiologi
- temporary_payment
- temporary_permintaan_lab
- temporary_permintaan_labmb
- temporary_permintaan_radiologi
- temporary_presensi
- temporary_radiologi
- temporary_resep
- temporary_resume
- temporary_sensus_harian
- temporary_surveilens_penyakit
- temporary_tambahan_potongan
- temporary_toko
- temppanggilnorawat
- temppanggilrm
- tampbeli1, tampjual1, tampjurnal, tampjurnal2
- tampreturbeli, tampreturjual, tampreturpiutang
- tamppiutang
```

**Solusi:** Ubah menjadi VIEW atau gunakan temporary table sesungguhnya di aplikasi.

**Estimasi Pengurangan: ~30 tabel**

---

### 3. TABEL DENGAN PREFIX SAMA (Bisa Dinormalisasi)

#### A. Tabel penilaian_* (85 tabel)
Banyak tabel penilaian dengan struktur mirip. Pertimbangkan untuk membuat tabel master:

```sql
-- Usulan struktur baru
CREATE TABLE penilaian (
    id INT PRIMARY KEY AUTO_INCREMENT,
    no_rawat VARCHAR(17),
    tanggal DATETIME,
    jenis_penilaian ENUM(
        'awal_keperawatan_gigi',
        'awal_keperawatan_igd',
        'awal_keperawatan_kebidanan',
        -- ... jenis lainnya
    ),
    -- kolom dinamis bisa disimpan di JSON atau tabel terpisah
    data JSON,
    created_at TIMESTAMP,
    FOREIGN KEY (no_rawat) REFERENCES reg_periksa(no_rawat)
);
```

Atau minimal kelompokkan berdasarkan similaritas:

```
KELOMPOK PENILAIAN KEPERAWATAN:
- penilaian_awal_keperawatan_gigi
- penilaian_awal_keperawatan_gigi_masalah
- penilaian_awal_keperawatan_igd
- penilaian_awal_keperawatan_igd_masalah
- penilaian_awal_keperawatan_kebidanan
- ... (30+ tabel)

KELOMPOK PENILAIAN MEDIS:
- penilaian_medis_igd
- penilaian_medis_ralan
- penilaian_medis_ralan_anak
- penilaian_medis_ralan_bedah
- ... (20+ tabel)

KELOMPOK SKRINING:
- skrining_adiksi_nikotin
- skrining_anemia
- skrining_curb65
- skrining_diabetes_melitus
- ... (34 tabel)
```

#### B. Tabel set_* (52 tabel) - Pengaturan/Konfigurasi
Sebagian besar adalah tabel setting yang seharusnya bisa digabung:

```
BISA DIGABUNG JADI 1 TABEL SETTING:
- set_akun, set_akun2, set_akun_bankbri, set_akun_bankjabar, etc.
- set_depo_ralan, set_depo_ranap
- set_harga_obat, set_harga_obat_ralan, set_harga_obat_ranap
- set_otomatis_tindakan_ralan, set_otomatis_tindakan_ralan_dokterpetugas
- set_validasi_catatan, set_validasi_registrasi

-> Usulan:
CREATE TABLE app_settings (
    id INT PRIMARY KEY AUTO_INCREMENT,
    kategori VARCHAR(50),
    nama_setting VARCHAR(100),
    nilai TEXT,
    deskripsi TEXT,
    UNIQUE KEY (kategori, nama_setting)
);
```

#### C. Tabel satu_sehat_* (49 tabel) - Integrasi Satu Sehat
Tabel integrasi dengan sistem Satu Sehat (Kemenkes RI):

```
MAPPING:
- satu_sehat_mapping_departemen
- satu_sehat_mapping_lab
- satu_sehat_mapping_lokasi_*
- satu_sehat_mapping_obat
- satu_sehat_mapping_radiologi
- satu_sehat_mapping_vaksin

DATA KLAIM:
- satu_sehat_careplan
- satu_sehat_condition
- satu_sehat_encounter
- satu_sehat_observation_*
- satu_sehat_procedure
- ... (banyak lagi)

-> Pertimbangkan arsitektur event-sourcing atau tabel log terpisah
```

#### D. Tabel surat_* (45 tabel) - Surat/Dokumen
```
SURAT KETERANGAN:
- surat_keterangan_sehat
- surat_keterangan_rawat_inap
- surat_keterangan_covid
- surat_keterangan_layak_terbang
- ... (banyak lagi)

-> Usulan:
CREATE TABLE surat (
    id INT PRIMARY KEY AUTO_INCREMENT,
    no_surat VARCHAR(20),
    jenis_surat VARCHAR(50),
    no_rawat VARCHAR(17),
    tanggal DATE,
    konten JSON,
    created_at TIMESTAMP
);
```

#### E. Tabel antri_* (Queue System)
```
ANTRIAN:
- antriapotek, antriapotek2, antriapotek3
- antriaps
- antrilabmb, antrilabmb2
- antrilabpa, antrilabpa2
- antrilabpk, antrilabpk2
- antrilayanankedokteranfisikrehabilitasi
- antrilayananprogramkfr
- antriloket, antriloketcetak
- antripelaksanaanedukasi
- antripemulangan
- antripengkajianrestrain
- antripenolakananjuranmedis
- antripenundaanpelayanan
- antripernyataanmemilihdpjp
- antripernyataanumum
- antripersetujuan*
- antripoli
- antriradiologi, antriradiologi2

-> Usulan: Sistem antrian terpusat
CREATE TABLE antrian (
    id INT PRIMARY KEY AUTO_INCREMENT,
    no_rawat VARCHAR(17),
    lokasi VARCHAR(20), -- 'apotek1', 'apotek2', 'lab_mb', 'radiologi', etc.
    status ENUM('menunggu','dipanggil','selesai'),
    nomor_antrian INT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
```

---

### 4. TABEL MASTER/REFERENCE (Perlu Dipertahankan)
Tabel-tabel ini adalah master data yang penting:

```
MASTER DATA INTI:
- pasien (data pasien)
- pegawai (data pegawai)
- dokter (data dokter)
- petugas (data petugas)
- poliklinik (data poliklinik)
- bangsal (data ruang/bangsal)
- kamar (data kamar)
- databarang (data obat/barang)
- penjab (penanggung jawab/asuransi)
- penyakit (data ICD-10)
- icd9 (data ICD-9)

MASTER DATA TRANSAKSI:
- reg_periksa (registrasi pemeriksaan)
- rawat_jl_dr, rawat_jl_pr, rawat_jl_drpr
- rawat_inap_dr, rawat_inap_pr, rawat_inap_drpr
- resep_dokter, resep_dokter_racikan
- detail_periksa_lab
- detail_periksa_labpa
```

---

### 5. TABEL BRIDGING/INTEGRASI (Perlu Review)
```
BPJS:
- bridging_dukcapil
- bridging_inhealth
- bridging_resep_apotek_bpjs
- bridging_rujukan_bpjs
- bridging_sep
- bridging_srb_bpjs
- bridging_surat_kontrol_bpjs
- aplicare_ketersediaan_kamar
- maping_dokter_dpjpvclaim
- maping_poli_bpjs
- rvp_klaim_bpjs

SATU SEHAT: (49 tabel)

INACBG: (11 tabel)

PCARE: (10 tabel)

INHEALTH: (8 tabel)

-> Evaluasi apakah semua tabel ini masih diperlukan atau bisa disederhanakan
```

---

## Usulan Skema Database Baru

### Target: Kurangi dari 1117 tabel ke ~300-400 tabel

```
KATEGORI TABEL BARU:

1. MASTER DATA (50 tabel)
   - pasien, pegawai, dokter, petugas
   - poliklinik, bangsal, kamar
   - databarang, datasuplier
   - penyakit, icd9
   - penjab, asuransi
   - dll.

2. TRANSAKSI (100 tabel)
   - reg_periksa (registrasi)
   - rawat_jalan (rawat jalan terpusat)
   - rawat_inap (rawat inap terpusat)
   - resep (resep terpusat)
   - pemeriksaan_lab
   - pemeriksaan_radiologi
   - dll.

3. ANTRIAN (5 tabel)
   - antrian (terpusat)
   - antrian_log

4. KEUANGAN (30 tabel)
   - akun, jurnal, rekening
   - piutang, bayar_piutang
   - pembelian, penjualan
   - dll.

5. INVENTARIS (15 tabel)
   - inventaris_barang
   - inventaris_pembelian
   - inventaris_pengeluaran
   - dll.

6. PENILAIAN/ASESMEN (20 tabel)
   - penilaian (master)
   - penilaian_detail

7. SKRINING (10 tabel)
   - skrining (master)
   - skrining_detail

8. SURAT/DOKUMEN (5 tabel)
   - surat (master)
   - surat_template

9. SETTING/KONFIGURASI (5 tabel)
   - app_settings
   - user_roles
   - user_permissions

10. BRIDGING/INTEGRASI (30 tabel)
    - bpjs_sep
    - bpjs_rujukan
    - satu_sehat_log
    - inacbg_klaim
    - dll.

11. AUDIT/LOG (10 tabel)
    - audit_log
    - user_activity
    - dll.
```

---

## Struktur Aplikasi Tauri yang Disarankan

```
simrs-khanza-tauri/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── database/    # Database layer
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs
│   │   │   ├── models/  # Data models
│   │   │   └── repositories/
│   │   ├── services/    # Business logic
│   │   ├── handlers/    # API handlers
│   │   └── utils/
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                 # Frontend (Svelte/React/Vue)
│   ├── routes/
│   ├── components/
│   ├── stores/         # State management
│   └── utils/
│
├── database/
│   ├── migrations/      # SQL migrations
│   ├── seeds/           # Initial data
│   └── schema.sql       # Optimized schema
│
└── docs/
    ├── API.md
    ├── DATABASE.md
    └── MIGRATION.md
```

---

## Prioritas Konversi Modul

### Fase 1: Core Module (Prioritas Tinggi)
1. **Autentikasi & Otorisasi**
   - login/logout
   - user management
   - role & permission

2. **Registrasi**
   - pendaftaran pasien baru
   - pendaftaran rawat jalan
   - pendaftaran rawat inap
   - antrian

3. **Master Data**
   - data pasien
   - data pegawai
   - data poliklinik/bangsal
   - data obat/barang

### Fase 2: Clinical Module
1. **Pemeriksaan**
   - rawat jalan
   - rawat inap
   - penilaian medis
   - penilaian keperawatan

2. **Farmasi**
   - resep
   - pelayanan obat
   - stok obat

3. **Laboratorium**
   - permintaan pemeriksaan
   - hasil pemeriksaan

4. **Radiologi**
   - permintaan pemeriksaan
   - hasil pemeriksaan

### Fase 3: Support Module
1. **Keuangan**
   - tagihan
   - pembayaran
   - piutang

2. **Inventaris**
   - pembelian
   - pengeluaran
   - stok

3. **Laporan**
   - laporan medis
   - laporan keuangan
   - laporan statistik

### Fase 4: Integration Module
1. **BPJS**
   - SEP
   - Rujukan
   - Klaim

2. **Satu Sehat**
   - Encounter
   - Observation
   - Condition

---

## Catatan Penting untuk AI

### 1. Foreign Keys
Banyak tabel menggunakan foreign key constraints. Perhatikan urutan:
- Buat tabel master dulu
- Baru buat tabel transaksi yang referensi ke master

### 2. Engine
Campuran MyISAM dan InnoDB. Disarankan semua pakai InnoDB untuk:
- Transaction support
- Foreign key constraints
- Row-level locking

### 3. Charset
Saat ini latin1_swedish_ci. Upgrade ke utf8mb4 untuk:
- Emoji support
- Karakter Unicode lengkap
- Collation yang lebih baik

### 4. Primary Keys
Perhatikan composite primary keys yang banyak digunakan:
```sql
-- Contoh
PRIMARY KEY (`no_rawat`, `tanggal`, `jam`)
```

### 5. Tabel yang TIDAK BOLEH DIHAPUS
```
TABEL INTI (JANGAN DIHAPUS):
- pasien
- pegawai
- dokter
- petugas
- reg_periksa
- poliklinik
- bangsal
- kamar
- databarang
- penjab
- penyakit
- rekening
- jurnal
```

---

## Query Helper untuk Analisis

```sql
-- Hitung jumlah tabel per prefix
SELECT
    SUBSTRING_INDEX(TABLE_NAME, '_', 1) as prefix,
    COUNT(*) as jumlah
FROM information_schema.TABLES
WHERE TABLE_SCHEMA = 'sik'
GROUP BY prefix
ORDER BY jumlah DESC;

-- Cari tabel dengan struktur mirip
SELECT TABLE_NAME, COLUMN_NAME, DATA_TYPE
FROM information_schema.COLUMNS
WHERE TABLE_SCHEMA = 'sik'
AND TABLE_NAME LIKE 'antri%'
ORDER BY TABLE_NAME, ORDINAL_POSITION;

-- Cari tabel kosong
SELECT TABLE_NAME, TABLE_ROWS
FROM information_schema.TABLES
WHERE TABLE_SCHEMA = 'sik'
AND TABLE_ROWS = 0
ORDER BY TABLE_NAME;

-- Cari tabel dengan prefix 'set_'
SELECT TABLE_NAME
FROM information_schema.TABLES
WHERE TABLE_SCHEMA = 'sik'
AND TABLE_NAME LIKE 'set_%'
ORDER BY TABLE_NAME;
```

---

## Kesimpulan

1. **Database saat ini: 1117 tabel** - terlalu banyak, perlu optimasi
2. **Target optimasi: 300-400 tabel** - pengurangan ~60-70%
3. **Masalah utama:**
   - Banyak tabel duplikat (numbered tables)
   - Banyak tabel temporary yang seharusnya bukan tabel permanen
   - Banyak tabel dengan struktur mirip yang bisa digabung
   - Kurang normalisasi

4. **Langkah optimasi:**
   a. Gabungkan tabel duplikat
   b. Ubah temporary tables menjadi views
   c. Normalisasi tabel dengan prefix sama
   d. Implementasikan soft delete
   e. Tambahkan audit trail terpusat

5. **Untuk konversi ke Tauri:**
   - Gunakan Rust untuk backend
   - Gunakan framework frontend modern (Svelte/React/Vue)
   - Implementasikan API layer yang bersih
   - Pisahkan logic dari UI

---

*Document ini dibuat untuk membantu AI memahami struktur proyek SIMRS Khanza*
*Last updated: 2026-02-14*