# Changelog MyDiskly Desktop 📦

Semua pembaruan penting, perbaikan bug, dan rilis fitur baru pada proyek **MyDiskly** akan dicatat dalam berkas ini. Format penomoran versi mengikuti prinsip Semantic Versioning.

---

## [1.0.2026] - 2026-10-28
### 🎉 Initial Public Release (Portable Edition)

#### ✨ Fitur Utama
- **Modern Fluent UI & Win32 Region Clipping**:
  - Desain minimalist Windows 11 Fluent Design dengan sudut rounded 30% (`CreateRoundRectRgn`) tanpa artefak black corner.
  - Skema warna clean monochrome: Dark Theme dan Light Theme dengan kontras ramah mata.
  - Typo smooth menggunakan sistem font Segoe UI Variable Text.
- **Realtime Storage Dashboard**:
  - Visualisasi kapasitas partisi penyimpanan dengan kode warna indikator dinamis: Putih/Normal, Kuning (70%), dan Merah (90%).
  - Telemetri I/O Read & Write Speed *real-time* langsung di footer dashboard.
- **S.M.A.R.T Hardware Diagnostics**:
  - Deteksi perangkat penyimpanan internal (NVMe, SSD, HDD).
  - Monitoring suhu drive, persentase wear life, status kesehatan hardware, dan durasi operasional (*power-on hours*).
- **Smart Disk Cleaner**:
  - Pembersih ruang penyimpanan cerdas 6 kategori solid in-place: Windows Recycle Bin, User Temp, DirectX Shader Cache, Crash Dumps, Windows System Temp, dan Windows Update Cache.
  - Seleksi cepat (*Select / Deselect All*) dengan indikasi estimasi ukuran dinamis.
  - Integrasi elevasi Administrator instan (*UAC restart*) untuk membersihkan direktori sistem terlindungi.
- **System Tray & Window Management**:
  - Mode *flyout* responsive di sudut kanan bawah dekat Taskbar notification area.
  - Single Instance Mutex (`Global\MyDiskly_SingleInstance_Mutex_Darkostepbro`) mencegah multi-proses.
  - Dukungan minimize ke System Tray dan toggle Run on Windows Startup.
- **Engine Architecture `disktrex.dll`**:
  - Logika telemetri dan cleaner dikemas dalam library binary DLL berperforma tinggi.
  - Fallback parser INI native tanpa dependensi eksternal.

---

## 🔮 Rencana Pembaruan Mendatang (Community Feedback Roadmap)

Daftar fitur berikut sedang dievaluasi berdasarkan kebutuhan dan masukan komunitas:
- [ ] Dukungan notifikasi ambang batas ruang penyimpanan (*Low Disk Space Toast Alert*).
- [ ] Grafik riwayat read/write I/O per drive secara visual.
- [ ] Deteksi dan integrasi pemantauan drive eksternal / USB flash drive secara hotplug.
- [ ] Lokalisasi multi-bahasa (Bahasa Indonesia & English).
- [ ] Pembersih cache per-browser tambahan (Chrome, Edge, Firefox).
