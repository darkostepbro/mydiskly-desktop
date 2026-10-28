# MyDiskly Desktop 💽

<p align="center">
  <img src="assets/icon.png" width="128" height="128" alt="MyDiskly Fluent Icon" />
</p>

<p align="center">
  <strong>Modern, Minimalist & Fast Windows 11 Storage Telemetry & Cleaner Suite</strong>
</p>

<p align="center">
  <a href="https://github.com/darkostepbro/mydiskly-desktop/releases"><img src="https://img.shields.io/badge/Release-v1.0.2026-0098FF?style=for-the-badge&logo=windows11&logoColor=white" alt="Release v1.0.2026" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="MIT License" /></a>
  <a href="https://github.com/darkostepbro/mydiskly-desktop"><img src="https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20(64--bit)-blue?style=for-the-badge&logo=windows" alt="Platform" /></a>
  <a href="https://github.com/darkostepbro/mydiskly-desktop/issues"><img src="https://img.shields.io/badge/Community%20Feedback-Welcome-orange?style=for-the-badge&logo=github" alt="Feedback" /></a>
</p>

[https://github.com/user-attachments/assets/55832364-6ee6-47f7-b2f1-20b5cbf4c8de](https://github.com/user-attachments/assets/e7f1822d-7977-4bc8-bfdd-9fc95263b5e8)


---

## 📖 Tentang MyDiskly

**MyDiskly** adalah aplikasi desktop modern portabel berbasis Python dan native Win32 API yang dirancang khusus untuk memantau kapasitas penyimpanan, kecepatan baca/tulis (*I/O Read & Write*), sensor kesehatan hardware (S.M.A.R.T), serta membersihkan file sampah sistem dengan antarmuka yang bersih (*clean monochrome*) dan elegan khas **Windows 11 Fluent Design**.

Aplikasi ini dikompilasi menjadi utilitas portabel mandiri tanpa installer rumit (*zero installation*), berjalan ringan di latar belakang dekat area notifikasi sistem (*System Tray*), serta ditenagai oleh mesin library terkompilasi **`disktrex.dll`**.

> **Publisher**: [Seroja Labs](https://serojalabs.sbs)  
> **Author**: [Darkostepbro](https://github.com/darkostepbro)  
> **Model**: Gemini 3.8 & Claude 4.6  
> **Watermark**: by Seroja Labs  
> **Versi Saat Ini**: `1.0.2026`  
> **Kategori**: Portable Utility Apps  
> **Status**: Aktif & Terus Diperbarui Berdasarkan Masukan Komunitas  

---

## ✨ Fitur Utama

- 🎨 **Windows 11 Fluent Monochrome UI**:
  - Tampilan modern dengan sudut *rounded 30%* presisi menggunakan Win32 Region Clipping (`CreateRoundRectRgn`) tanpa artefak sudut hitam (*zero black corners*).
  - Pilihan tema **Dark** & **Light** yang nyaman di mata dengan kontras clean.
  - Tipografi halus menggunakan font sistem native `Segoe UI Variable Text`.

- ⚡ **Realtime Storage Dashboard & I/O Monitor**:
  - Pemantauan kapasitas partisi drive internal dan eksternal secara *real-time*.
  - Indikator bar dinamis berstandar ergonomis:
    - ⚪ **Normal** (`< 70%`): Putih (Dark Mode) / Netral (Light Mode).
    - 🟡 **Peringatan** (`70% - 89%`): Kuning Amber.
    - 🔴 **Kritis** (`≥ 90%`): Merah.
  - Telemetri kecepatan baca/tulis (*Read/Write Speed*) langsung di bagian bawah dashboard.

- 📊 **S.M.A.R.T Hardware Diagnostics**:
  - Deteksi fisik penyimpanan (NVMe, SSD, HDD).
  - Pembacaan sensor suhu drive (*Temperature*), persentase masa pakai (*Wear Life / Health Percentage*), status kelayakan fisik (*Health Status*), dan durasi aktif (*Power-On Hours*).

- 🧹 **Smart Disk Cleaner (Resilient In-Place Architecture)**:
  - Pembersihan 6 kategori file sampah Windows yang aman:
    1. **Windows Recycle Bin**
    2. **User Temp Files (`%TEMP%`)**
    3. **DirectX Shader Cache**
    4. **Crash Error Dumps**
    5. **Windows System Temp** *(memerlukan hak Administrator)*
    6. **Windows Update Cache (`SoftwareDistribution`)** *(memerlukan hak Administrator)*
  - Fitur *Pilih Semua / Deselect All* sekali klik dan kalkulasi ukuran dinamis secara *real-time*.
  - Tombol **⚡ Restart as Admin** instan untuk membuka akses penuh ke cache sistem terlindungi.

- 🛡️ **Single-Instance Mutex Protection**:
  - Dilengkapi proteksi Win32 kernel mutex `Global\MyDiskly_SingleInstance_Mutex_Darkostepbro`.
  - Mencegah aplikasi terbuka lebih dari 1 kali saat user/admin melakukan double-click berulang atau membuka lewat elevated context; instance yang sedang berjalan otomatis difokuskan ke depan layar.

- 🎛️ **Pengaturan & Integrasi Sistem Fleksibel**:
  - Opsi *Run on Windows Startup* dengan integrasi Registry langsung.
  - Opsi *Minimize to System Tray* (ikon taskbar flyout responsif).
  - Seluruh konfigurasi tersimpan rapi dalam format portabel [mydiskly.ini](file:///C:/Users/DARKO/.gemini/antigravity/scratch/mydiskly-desktop/mydiskly.ini).

---

## 🏛️ Arsitektur Proyek

```
┌──────────────────────────────────────────────────────────────────┐
│                     MyDiskly UI Frontend                         │
│       (CustomTkinter + Windows 11 Fluent Theme Engine)           │
│                                                                  │
│  [ Dashboard ]      [ SMART Sensors ]      [ Disk Cleaner ]      │
└─────────────────────────────────┬────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│                 Native Win32 Integration Layer                   │
│   • SetWindowRgn (30% Smooth Rounding & Alpha Clipping)         │
│   • Single-Instance Kernel Mutex Protection                      │
│   • SystemParametersInfoW Flyout Positioning                     │
│   • pystray Background Notification Daemon                       │
└─────────────────────────────────┬────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│                disktrex.dll Core Engine                          │
│   • Storage Partition Telemetry & Realtime Disk I/O Rate         │
│   • WMI / PowerShell S.M.A.R.T NVMe/SSD Sensor Parser            │
│   • Multi-Target Cache Analyzer & Multi-Threaded Cleaner         │
│   • Resilient Fallback INI Configuration Engine                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 🚀 Panduan Penggunaan & Menjalankan Aplikasi

### Opsi 1: Menggunakan Rilis Siap Pakai (Portable)
1. Unduh paket terbaru dari [GitHub Releases](https://github.com/darkostepbro/mydiskly-desktop/releases).
2. Ekstrak folder `mydiskly-v1` ke lokasi mana pun (misalnya `D:\PortableApps\MyDiskly` atau flashdisk).
3. Jalankan **`MyDiskly.exe`**. Aplikasi langsung siap digunakan tanpa memerlukan instalasi apa pun!

---

### Opsi 2: Menjalankan Langsung dari Source Code (Development)

Pastikan sistem Anda telah terpasang **Python 3.10+ (64-bit)** dan **Git**.

```powershell
# 1. Kloning repositori ini
git clone https://github.com/darkostepbro/mydiskly-desktop.git
cd mydiskly-desktop

# 2. Buat dan aktifkan virtual environment (opsional)
python -m venv venv
.\venv\Scripts\Activate.ps1

# 3. Pasang semua pustaka yang dibutuhkan
pip install -r requirements.txt

# 4. Jalankan aplikasi dalam mode pengembangan
python src/main.py
```

---

### 🔨 Cara Membangun Executable (.exe & .dll) Mandiri

Repositori ini menyertakan skrip otomatisasi build yang lengkap dan modular di folder `scripts/`:

```powershell
# 1. Mengompilasi engine disktrex.dll
python scripts/build_dll.py

# 2. Mengompilasi paket portabel lengkap (MyDiskly.exe + lib/)
python scripts/build_exe.py
```

Setelah proses selesai, paket rilis siap pakai akan terbentuk di direktori:
```text
dist/
└── mydiskly-v1/
    ├── assets/
    ├── lib/
    ├── app_icon.png
    ├── disktrex.dll
    ├── MyDiskly.exe
    └── mydiskly.ini
```

---

## 📁 Struktur Direktori Repositori

```text
mydiskly-desktop/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.yml         # Template pelaporan bug terstruktur
│   │   └── feature_request.yml    # Template usulan fitur komunitas
│   └── workflows/
│       └── build.yml              # CI workflow build otomatis
├── assets/
│   ├── app_icon.png               # Asset PNG brand resolusi tinggi
│   ├── icon.ico                   # Multi-resolution Windows ICO (16-256px)
│   └── icon.png                   # Fluent 3D Icon dengan pure alpha transparency
├── scripts/
│   ├── build_dll.py               # Tool kompilasi byte-engine disktrex.dll
│   └── build_exe.py               # Tool bundling portable MyDiskly.exe
├── src/
│   ├── __init__.py                # Package initialization
│   ├── disktrex_engine.py         # Source code engine telemetri & cleaner
│   └── main.py                    # Source code utama UI & interaksi Win32
├── .gitignore                     # Git filter standar Python & Windows
├── CHANGELOG.md                   # Catatan rilis v1.0 dan histori update
├── CONTRIBUTING.md                 # Panduan berkontribusi bagi komunitas
├── LICENSE                        # Lisensi resmi MIT
├── mydiskly.ini                   # Berkas konfigurasi bawaan aplikasi
├── README.md                      # Dokumentasi komprehensif proyek
└── requirements.txt               # Daftar dependensi modul Python
```

---

## 💬 Siklus Pembaruan & Feedback Komunitas

Proyek ini dibangun untuk komunitas! Kami mendengarkan setiap saran, kendala teknis, dan ide pembaruan dari pengguna.

### Bagaimana alur pembaruan bekerja?
1. **Ajukan Permintaan**: Buat [Issue Baru](https://github.com/darkostepbro/mydiskly-desktop/issues) untuk mengusulkan ide sensor atau melaporkan bug.
2. **Diskusi & Evaluasi**: Ide yang diajukan akan ditinjau dan diberi label (misal: `enhancement`, `community-request`).
3. **Pengembangan & Rilis Berkala**: Fitur yang telah disetujui akan diimplementasikan ke branch pengembangan dan dirilis secara berkala pada tag versi berikutnya di [Releases](https://github.com/darkostepbro/mydiskly-desktop/releases).

Lihat [CONTRIBUTING.md](CONTRIBUTING.md) untuk panduan lebih terperinci mengenai cara berkontribusi.

---

## 📜 Lisensi & Atribusi

Proyek ini dirilis secara terbuka di bawah lisensi [MIT License](LICENSE).

```text
Copyright (c) 2026 Seroja Labs & Darkostepbro.
Publisher: Seroja Labs
Author: Darkostepbro
Model: Gemini 3.8 & Claude 4.6
Repository: https://github.com/darkostepbro/mydiskly-desktop
Feedback: https://github.com/darkostepbro/mydiskly-desktop
Made in Indonesia.
```

Silakan gunakan, pelajari, dan modifikasi kode sumber ini secara bebas dengan tetap menyertakan atribusi hak cipta asli.
