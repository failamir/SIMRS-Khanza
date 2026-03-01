# Panduan Struktur Kode Java SIMRS Khanza

## Arsitektur Aplikasi Saat Ini

### Bahasa & Framework
- **Bahasa:** Java
- **GUI:** Swing (JFrame, JDialog, JPanel)
- **Database:** MySQL/MariaDB
- **Build Tool:** NetBeans project (ant)
- **Arsitektur:** Monolithic Desktop Application

---

## Struktur Folder Utama

```
SIMRS-Khanza/
├── src/                          # Source code utama
│   ├── simrskhanza/              # Main application (frmUtama.java)
│   ├── fungsi/                   # Utility functions
│   │   ├── koneksiDB.java        # Database connection
│   │   ├── sekuel.java           # SQL query helpers
│   │   └── ...
│   ├── widget/                   # Custom Swing components
│   ├── bridging/                 # External integrations (BPJS, etc.)
│   ├── rekammedis/               # Medical records module
│   ├── keuangan/                 # Finance module
│   ├── inventory/                # Inventory/pharmacy module
│   ├── laporan/                  # Reports module
│   ├── surat/                    # Letters/documents module
│   ├── kepegawaian/              # HR module
│   ├── setting/                  # Settings module
│   ├── inventaris/               # Asset management
│   ├── dapur/                    # Kitchen/diet module
│   ├── ipsrs/                    # IPSRS module
│   ├── toko/                     # Shop module
│   ├── parkir/                   # Parking module
│   ├── perpustakaan/             # Library module
│   ├── tranfusidarah/            # Blood transfusion module
│   └── ...
│
├── KhanzaAntrianApotek/          # Pharmacy queue display app
├── KhanzaAntrianLoket/           # Registration queue display app
├── KhanzaAntrianLoket2/          # Registration queue display app (v2)
├── KhanzaAntrianPoli/            # Polyclinic queue display app
├── KhanzaCetakAntrianLoket/      # Queue ticket printer app
│
├── KhanzaHMSAnjungan/            # Self-service kiosk app
├── KhanzaHMSAnjunganFingerPrint/ # Kiosk with fingerprint
├── KhanzaHMSAutoVerify/          # Auto verification service
├── KhanzaHMSAutoVerify2/         # Auto verification service (v2)
├── KhanzaHMSResume/              # Resume service
│
├── KhanzaHMSServiceAplicare/     # BPJS Aplicare service
├── KhanzaHMSServiceMandiri/      # Bank Mandiri service
├── KhanzaHMSServiceMobileJKN/    # BPJS Mobile JKN service
├── KhanzaHMSServiceMobileJKNERM/ # BPJS Mobile JKN ERM service
├── KhanzaHMSServiceMobileJKNFKTP/# BPJS Mobile JKN FKTP service
├── KhanzaHMSServicePCare/        # BPJS PCare service
├── KhanzaHMSServiceSIRSYankes/   # SIRS Yankes service
├── KhanzaHMSServiceSPDGT/        # SPDGT service
├── KhanzaHMSServiceSatuSehat/    # Satu Sehat service
│
├── KhanzaPengenkripsiTeks/       # Text encryption tool
├── KhanzaSecurity16bit/          # Security tool
├── KhanzaUpdater/                # Auto updater
│
├── api-bpjsfktl/                 # BPJS FKTL API
├── api-bpjsfktp/                 # BPJS FKTP API
├── api-bridgingradiologi/        # Radiology bridging API
│
├── alarm/                        # Alarm/sound files
├── bankjateng/                   # Bank Jateng integration
├── bankpapua/                    # Bank Papua integration
├── bjb/                          # BJB bank integration
├── mandiri/                      # Bank Mandiri integration
│
├── edokter/                      # e-Doctor module
├── emcu/                         # e-MCU module
├── epasien/                      # e-Patient module
├── esign/                        # e-Sign module
├── presensi/                     # Attendance module
├── resumepasien/                 # Patient resume module
│
├── webapps/                      # Web applications
├── suara/                        # Sound files
├── gambar/                       # Image files
├── gambarradiologi/              # Radiology images
├── driverFlexCode/               # Fingerprint driver
│
├── lib/                          # External libraries (.jar)
├── report/                       # Report templates (.jasper)
├── setting/                      # Configuration files
├── cache/                        # Cache files
└── build/                        # Build output
```

---

## Modul Aplikasi Utama

### 1. Modul Inti (simrskhanza/)
```
frmUtama.java - Main frame (menu utama)
```

### 2. Modul Rawat Jalan (src/rekammedis/)
```
- Pendaftaran rawat jalan
- Pemeriksaan dokter
- Penilaian keperawatan
- Resume medis
- dll.
```

### 3. Modul Rawat Inap (src/rekammedis/)
```
- Pendaftaran rawat inap
- Pemeriksaan harian
- Catatan keperawatan
- Penilaian medis
- dll.
```

### 4. Modul Farmasi (src/inventory/)
```
- Data obat/barang
- Resep dokter
- Pelayanan obat
- Stok obat
- Pembelian
- dll.
```

### 5. Modul Keuangan (src/keuangan/)
```
- Tagihan
- Pembayaran
- Piutang
- Jurnal keuangan
- Rekening bank
- dll.
```

### 6. Modul Bridging (src/bridging/)
```
- BPJS (SEP, Rujukan, Klaim)
- Satu Sehat
- Aplicare
- INACBG
- Bank integrations
- dll.
```

---

## Koneksi Database

### File Konfigurasi: setting/database.xml
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE properties SYSTEM "http://java.sun.com/dtd/properties.dtd">
<properties>
    <entry key="HOST">encrypted_host</entry>
    <entry key="PORT">encrypted_port</entry>
    <entry key="DATABASE">encrypted_db</entry>
    <entry key="USER">encrypted_user</entry>
    <entry key="PAS">encrypted_password</entry>
</properties>
```

### Koneksi Database (fungsi/koneksiDB.java)
```java
// Koneksi menggunakan MysqlDataSource
// Host, Port, Database, User, Password dienkripsi dengan AES
// Connection pooling sederhana
```

---

## Widget/Component Library

Custom Swing components di folder `widget/`:

| Komponen | Deskripsi |
|----------|-----------|
| Button.java | Tombol custom |
| TextBox.java | Text field custom |
| ComboBox.java | Dropdown custom |
| Table.java | Tabel custom |
| Panel.java | Panel custom |
| ScrollPane.java | Scroll pane custom |
| TabPane.java | Tab panel custom |
| Tanggal.java | Date picker |
| CekBox.java | Checkbox custom |
| RadioButton.java | Radio button custom |
| PasswordBox.java | Password field |
| TextArea.java | Text area custom |
| Label.java | Label custom |
| MenuBar.java | Menu bar custom |
| Toolbar.java | Toolbar custom |
| ProgressBar.java | Progress bar custom |
| RunText.java | Running text (untuk antrian) |
| InternalFrame.java | Internal frame |
| Desktop.java | Desktop pane |
| PanelGlass.java | Panel dengan efek glass |
| panelisi.java | Panel custom |

---

## Aplikasi Pendukung (Services)

### Antrian Display Apps
- **KhanzaAntrianApotek** - Display antrian farmasi
- **KhanzaAntrianLoket** - Display antrian loket pendaftaran
- **KhanzaAntrianPoli** - Display antrian poliklinik

### BPJS Services
- **KhanzaHMSServiceAplicare** - Layanan ketersediaan kamar
- **KhanzaHMSServiceMobileJKN** - Integrasi Mobile JKN
- **KhanzaHMSServicePCare** - Integrasi PCare
- **KhanzaHMSServiceSatuSehat** - Integrasi Satu Sehat

### Bank Services
- **KhanzaHMSServiceMandiri** - Integrasi Bank Mandiri
- bankjateng, bankpapua, bjb - Integrasi bank lain

### Kiosk & Verification
- **KhanzaHMSAnjungan** - Kiosk mandiri
- **KhanzaHMSAnjunganFingerPrint** - Kiosk dengan sidik jari
- **KhanzaHMSAutoVerify** - Verifikasi otomatis

---

## Pola Kode yang Umum

### 1. Form Dialog (Dlg*.java)
```java
public class DlgPasien extends JDialog {
    // Komponen GUI
    private TextBox txtNoRm;
    private TextBox txtNama;
    private Button btnSimpan;
    private Button btnHapus;
    private Table table;

    // Metode
    public void tampil() { /* tampilkan data */ }
    public void simpan() { /* simpan data */ }
    public void hapus() { /* hapus data */ }
    public void cari() { /* cari data */ }
}
```

### 2. Database Query Pattern
```java
// Menggunakan sekuel.java helper
sekuel.query("SELECT * FROM pasien WHERE no_rkm_medis=?", noRm);

// Atau direct JDBC
PreparedStatement ps = conn.prepareStatement(sql);
ps.setString(1, value);
ResultSet rs = ps.executeQuery();
```

### 3. Internal Frame Pattern
```java
public class frmUtama extends JFrame {
    private JDesktopPane desktopPane;

    public void showPasien() {
        DlgPasien pasien = new DlgPasien(null, false);
        desktopPane.add(pasien);
        pasien.setVisible(true);
    }
}
```

---

## File Konfigurasi

### setting/database.xml
Konfigurasi database (dienkripsi AES)

### setting/setting.xml
Pengaturan umum aplikasi

### report/*.jasper
Template laporan (JasperReport)

---

## Untuk Konversi ke Tauri

### Yang Perlu Diperhatikan:

1. **GUI Swing -> Web UI**
   - Semua JFrame -> React/Vue/Svelte component
   - Semua JDialog -> Modal component
   - Semua JTable -> Data table component
   - Semua widget custom -> UI library

2. **Database Layer**
   - JDBC -> SQLx (Rust) atau ORM
   - Connection pooling -> deadpool/r2d2

3. **Business Logic**
   - Pindahkan ke Rust backend
   - Buat API layer

4. **Reports**
   - JasperReport -> Alternatif (HTML report, PDF generator)

5. **Services**
   - Java services -> Rust async tasks
   - HTTP client -> reqwest (Rust)

6. **Security**
   - AES encryption -> Rust crypto libraries

---

## Jumlah File per Modul

```
src/bridging/     : 469 file (integrasi eksternal)
src/rekammedis/   : 476 file (rekam medis)
src/keuangan/     : 335 file (keuangan)
src/inventory/    : 237 file (farmasi/inventori)
src/laporan/      : 216 file (laporan)
src/kepegawaian/  : 148 file (kepegawaian)
src/inventaris/   : 72 file (aset)
src/surat/        : 80 file (surat)
src/toko/         : 71 file (toko)
src/dapur/        : 73 file (dapur/diet)
src/ipsrs/        : 89 file (IPSRS)
src/setting/      : 68 file (pengaturan)
src/fungsi/       : 20 file (utility)
src/widget/       : 37 file (komponen)
```

---

*Document ini membantu AI memahami struktur kode Java untuk konversi ke Tauri*