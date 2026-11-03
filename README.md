# MyDiskly (Pure Native Rust Edition) 💽
> **A Super Lightweight, Modern & Fluent Windows 11 & 10 Storage Scanner, Telemetry & Cleaner Suite**

<p align="center">
  <img src="assets/icon.png" width="128" height="128" alt="MyDiskly Native Icon" />
</p>

<p align="center">
  <a href="https://github.com/darkostepbro/mydiskly-desktop/releases"><img src="https://img.shields.io/badge/Release-v1.1.2026-0B65E6?style=for-the-badge&logo=windows11&logoColor=white" alt="Release v1.1.2026" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-GPL--3.0-green?style=for-the-badge" alt="GPL-3.0 License" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Language-Rust%202021-DEA584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust 2021" /></a>
  <a href="https://github.com/darkostepbro/mydiskly-desktop"><img src="https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20(x64)-blue?style=for-the-badge&logo=windows" alt="Platform" /></a>
  <a href="https://github.com/darkostepbro/mydiskly-desktop/issues"><img src="https://img.shields.io/badge/Community%20Feedback-Welcome-orange?style=for-the-badge&logo=github" alt="Feedback" /></a>
</p>

---

## 📖 Tentang MyDiskly
**MyDiskly** adalah aplikasi diagnostik penyimpanan (*storage telemetry*), pemantauan kesehatan drive (S.M.A.R.T), audit spesifikasi perangkat keras, dan pembersih sampah sistem (*smart disk cleaner*) yang dibangun **100% murni menggunakan bahasa pemrograman Rust** (`eframe` / `egui 0.29` + `Glow native renderer`).

MyDiskly dirancang khusus untuk pengguna Windows 10 & Windows 11 (termasuk Windows edisi modifikasi seperti *Ghost Spectre*, *AtlasOS*, *Tiny11*, maupun *ReviOS*) yang menginginkan aplikasi utilitas dengan **konsumsi memori ultra-rendah**, tampilan *docking* vertikal modern (*monochrome clean UI*), tanpa dependensi Python, tanpa WebView2/Chromium runtime, dan tanpa overhead JavaScript.

> **Publisher**: [Seroja Labs](https://serojalabs.sbs/mydiskly)  
> **Author**: [Darkostepbro](https://github.com/darkostepbro)  
> **Model / AI Pair**: Google DeepMind Advanced Agentic AI  
> **Versi**: `1.1.2026`  
> **Kategori**: Native Installer & Portable Storage Suite  
> **Lisensi**: GNU General Public License v3.0 (GPL-3.0)  

---

## 🚀 Perbandingan: Versi Lama (Python) vs Versi Baru (Pure Rust)

| Parameter | Versi Lama (Python / PyInstaller) | Versi Baru (Pure Native Rust) | Peningkatan |
| :--- | :--- | :--- | :--- |
| **Bahasa Pemrograman** | Python 3.11 + Tkinter/ctypes | **100% Rust 2021 (Win32 + Glow)** | Zero Runtime Overhead |
| **Ukuran Memori (RAM)** | ~110 MB – 180 MB | **~30 MB – 40 MB** (Aktif) / **~3–5 MB** (Tray) | **Hemat RAM hingga 80%** |
| **Waktu Startup** | 2.5 – 4.0 detik (Ekstraksi zip) | **< 50 milidetik (Instan)** | **50x Lebih Cepat** |
| **Dependensi Eksternal** | Python DLLs, WebView2, PyInstaller | **Nol (Zero External Dependencies)** | Portabel & Mandiri |
| **Kompatibilitas OS** | Sering gagal di Ghost Spectre/Lite | **Windows 10 & 11 x64 (Semua Edisi)** | 100% Stabil & Tangguh |
| **Arsitektur Binary** | Bundel script Python | **Native PE x64 Terkompilasi** | Aman, Cepat & Efisien |

---

## ✨ Fitur-Fitur Utama

### 1. 🖥️ Vertical Right-Docking & Native Acrylic Dark Blur
- Menempel vertikal secara presisi di layar sisi kanan (`SPI_GETWORKAREA`) dengan lebar **420 px**.
- Mode **Always-On-Top** dengan tombol minimal **Close `[x]`** dan **Minimize `[—]`**.
- Efek visual *Dark Blur / Acrylic* asli Windows menggunakan `SetWindowCompositionAttribute` dan `DwmSetWindowAttribute` (`DWMWCP_ROUND`).
- Multi-bahasa instan **`[ID | EN]`** serta label izin akses dinamis **`[User mode]`** (redup) dan **`[Admin mode]`** (terang).

### 2. 💽 Partisi Interaktif & Telemetri Real-Time
- **Interactive Partition Cards**: Menampilkan bar kapasitas disk berstandar ergonomis (Normal / Warning / Critical). Mengklik kartu partisi membuka menu aksi:
  1. `1. Open file` (Membuka drive di Windows File Explorer)
  2. `2. Properties` (Membuka dialog Drive Properties native)
- **S.M.A.R.T Drive Diagnostics**: Menampilkan model drive fisik, media type (NVMe, SSD, HDD), bus type, **Persentase Kesehatan (Bold %)**, **Suhu Sensor (Bold °C)**, dan durasi aktif (*Power-On Hours*).
- **Docked Read / Write Waveform Graph**: Kanvas grafik sub-pixel 60 FPS untuk akselerasi kecepatan baca (Hijau) dan tulis (Biru) dengan buffer riwayat 30 detik.

### 3. 🧹 Smart Cleaner & Browser Cache Isolation
- **Pembersihan Cache Browser Terisolasi**: Mendeteksi Google Chrome, Brave Browser, dan Microsoft Edge secara otomatis.
- **Jaminan 100% Aman**: Hanya membersihkan *System Cache, Code Cache, GPUCache, Service Worker Cache*. **TIDAK PERNAH** menghapus history, bookmark, cookie, atau sesi login.
- **System Cache Shield**: Proteksi item cache tingkat admin. Jika tidak dijalankan sebagai Administrator, opsi cache admin otomatis *read-only* dengan tombol elevasi sekali klik.
- **Animasi Proses & Popup Selesai**: Animasi progress bar saat pembersihan dan modal dialog pop-up konfirmasi pembersihan permanen dengan latar *softblur*.

### 4. ⚙️ Spesifikasi Hardware & Sistem Notifikasi
- **Tabel Spesifikasi Hardware Responsif**: Menampilkan 15 parameter teknis riil (OS Build, CPU, GPU, RAM, Motherboard, BIOS Mode, Storage, Audio, Peripheral, Jaringan, dll.) tanpa scroll horizontal.
- **Cache Notification Alert**: Pengingat ambang batas pembersihan otomatis (`500MB`, `800MB`, `1GB`, `2GB`, `Custom MB/GB` dengan auto-rollover >999MB, `Never`).
- **Sound Effect Terpasang**: Notifikasi bersuara native dengan file audio `interface.wav`.
- **System Notification & Focus Assist Aware**: Otomatis mendeteksi status aktivasi Windows Toast Notification.

### 5. 📁 Komponen Konfigurasi & Persistence Mandiri
Aplikasi secara otomatis menghasilkan dan menyinkronkan 3 file konfigurasi:
- **`mydiskly.ini`**: Preferensi tema, bahasa, startup registry, dan konfigurasi ambang batas notifikasi.
- **`myspecs.ini`**: Diawali header `[{user} - {computer}]` dan 15 baris spesifikasi teknis lengkap. Otomatis diperbarui saat terjadi pergantian hardware atau deteksi hotplug USB.
- **`historydiskhealth.ini`**: Menyimpan riwayat tanggal pengecekan kesehatan partisi (`[02/nov/2026]`, dll.) secara terstruktur.

### 6. 📦 Custom Installer & Uninstaller Suite (`480 × 300` px)
- Biner instalasi mandiri (**`MyDiskly-Setup.exe`**) dan pencopot pemasangan (**`Uninstall.exe`**).
- **3-Page Roadmap Capsule**: `1. Welcome` ➔ `2. Installing` ➔ `3. Complete` dengan indikator vector checkmark murni.
- **Full-Rounded Pill Toggles**: `Create the shortcut`, `Run as Administrator`, `Pin on Start Menu`, `Open File Location`.
- **100% Background Silent Execution**: Menghilangkan kedipan jendela CMD via Win32 `CREATE_NO_WINDOW` & `ShellExecuteW`.
- **Folder Instalasi Bersih**: Hanya memuat 6 file resmi di `%LOCALAPPDATA%\Programs\MyDiskly`:
  1. `MyDiskly.exe`
  2. `Uninstall.exe`
  3. `mydiskly.ini`
  4. `myspecs.ini`
  5. `historydiskhealth.ini`
  6. `LICENSE.txt`
- **Permanent Uninstall**: Menghentikan paksa aplikasi yang sedang berjalan secara instan (meskipun di tray atau elevated) lewat 4 lapis terminasi, membuka kunci folder, dan menghapus seluruh file tanpa sisa.

---

## 🏛️ Struktur Workspace Cargo

```
mydiskly-rust/
├── Cargo.toml                         # Root Cargo Workspace
├── Cargo.lock                         # Locked Dependencies
├── crates/
│   └── mydiskly-core/                 # Shared Win32 Kernel & Telemetry Library
│       ├── src/
│       │   ├── lib.rs                 # Core Crate Exports
│       │   ├── gatekeeper.rs          # Windows 10 & 11 OS Gatekeeping
│       │   ├── disk_telemetry.rs      # Win32 Partition Stats & I/O 30s Buffer
│       │   ├── smart_diagnostics.rs   # Physical Drive S.M.A.R.T Sensors
│       │   ├── hotplug_monitor.rs     # USB Hotplug Real-time Poller
│       │   ├── hardware_specs.rs      # 15-Parameter Hardware Audit
│       │   ├── cleaner_engine.rs      # Multi-category Cache Engine & Browser Isolation
│       │   ├── ini_manager.rs         # mydiskly.ini, myspecs.ini & historydiskhealth.ini
│       │   └── models/                # Telemetry & Config Structs
├── apps/
│   ├── mydiskly-app/                  # Main GUI Application (Right-Docked)
│   │   ├── src/
│   │   │   ├── main.rs                # Docking Loop, Visuals, App Core
│   │   │   ├── icons.rs               # Vector Monochrome Icons
│   │   │   ├── sys_integration.rs     # Win32 Tray, Dark Blur, Registry, Audio
│   │   │   └── views/
│   │   │       ├── dashboard.rs       # Partitions, SMART Cards & I/O Graph
│   │   │       ├── cleaner.rs         # Cleaner UI, Progress & Modal Dialog
│   │   │       ├── specs.rs           # Specs Table & Settings
│   │   │       └── about.rs           # About Card, Hyperlinks & Watermark
│   │   └── assets/                    # Icon PE Resources & interface.wav
│   └── mydiskly-installer/            # 3-Page Roadmap Setup & Uninstaller Suite
│       └── src/
│           └── main.rs                # Setup Wizard, Uninstaller, Silent Shortcuts
└── assets/                            # Brand Assets & Application Icons
```

---

## 🛠️ Panduan Kompilasi & Build

### Prasyarat
- **Sistem Operasi**: Windows 10 / Windows 11 (64-bit)
- **Rust Toolchain**: `stable-x86_64-pc-windows-gnu` atau `stable-x86_64-pc-windows-msvc` (Rust 1.75+)
- **LLVM-MinGW / GCC**: Untuk pengikatan pustaka Win32 dan kompilasi resource icon (`windres`)

### Kompilasi Seluruh Workspace
Buka PowerShell di direktori root repository, lalu jalankan:

```powershell
# 1. Kompilasi Aplikasi Utama
cargo build --release -p mydiskly-app

# 2. Kompilasi Custom Installer (otomatis memaketkan MyDiskly.exe)
cargo build --release -p mydiskly-installer
```

Hasil biner terkompilasi akan berada di folder `target/release/`:
* `target/release/MyDiskly.exe` (Aplikasi utama)
* `target/release/MyDiskly-Setup.exe` (Installer wizard)
* `target/release/Uninstall.exe` (Uninstaller suite)

---

## 📄 Lisensi
Proyek ini dilisensikan di bawah **GNU General Public License v3.0 (GPL-3.0)**. Anda bebas menggunakan, memodifikasi, dan mendistribusikan kode sumber ini dengan tetap menyertakan atribusi hak cipta dan lisensi terbuka yang sama.

Copyright (C) 2026 **Seroja Labs** & **Darkostepbro**. All rights reserved.  
Official Website: [serojalabs.sbs/mydiskly](https://serojalabs.sbs/mydiskly)  
Source Code: [github.com/darkostepbro/mydiskly-desktop](https://github.com/darkostepbro/mydiskly-desktop)
